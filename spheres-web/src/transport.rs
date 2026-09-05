//! Campaign-bound request receipts. Transport identity stays outside the simulation.
use crate::*;

/// Shared by the HTTP route and regressions, including response-loss retries.
#[derive(Debug)]
pub(crate) struct AdvanceError {
    pub message: String,
    pub requires_review: bool,
}
impl From<String> for AdvanceError {
    fn from(message: String) -> Self {
        Self {
            message,
            requires_review: false,
        }
    }
}
impl From<&str> for AdvanceError {
    fn from(message: &str) -> Self {
        message.to_string().into()
    }
}
pub(crate) fn advance_request(
    g: &mut Game,
    payload: &serde_json::Value,
) -> Result<serde_json::Value, AdvanceError> {
    if let Some(session) = payload.get("session_id") {
        if session.as_str() != Some(g.session_id.as_str()) {
            return Err(AdvanceError { message: "This campaign changed or the server restarted. Review the current campaign; the earlier turn's outcome cannot be inferred here.".into(), requires_review: true });
        }
    }
    let token = match (payload.get("client_id"), payload.get("request_seq")) {
        (None, None) => None, // older clients retain their existing API
        (Some(client), Some(seq)) => {
            let client = client
                .as_str()
                .filter(|s| !s.is_empty() && s.len() <= 96)
                .ok_or("Invalid browser request identity.")?;
            let seq = seq
                .as_u64()
                .filter(|n| *n > 0)
                .ok_or("Invalid browser request sequence.")?;
            if payload.get("session_id").and_then(|v| v.as_str()).is_none() {
                return Err("A protected turn requires its campaign identity.".into());
            }
            if let Some((last, previous, why)) = g.advance_receipts.get(client) {
                if seq == *last {
                    if previous != payload {
                        return Err("That turn identity already belongs to different orders. Nothing was advanced.".into());
                    }
                    return Ok(state_json(g, why.clone()));
                }
                if seq < *last {
                    return Err(AdvanceError { message: "A newer turn from this browser identity was already processed. Review the current state; these earlier orders will not be automatically replayed.".into(), requires_review: true });
                }
            } else if g.advance_receipts.len() >= 256 {
                return Err(
                    "This campaign has too many browser sessions. Continue in an existing tab."
                        .into(),
                );
            }
            Some((client.to_owned(), seq))
        }
        _ => return Err("A protected turn needs both a browser identity and sequence.".into()),
    };
    let commands = advance_commands(&g.world, payload)?;
    let days = asked_days(payload)?;
    let months = if days.is_none() {
        Some(asked_months(payload)?)
    } else {
        None
    };
    let (_, why) = if let Some(days) = days {
        g.advance_days(days as usize, commands)
    } else {
        g.advance_months(months.unwrap() as usize, commands)
    };
    if let Some((client, seq)) = token {
        g.advance_receipts
            .insert(client, (seq, payload.clone(), why.clone()));
    }
    Ok(state_json(g, why))
}

/// Every immediate order may be safely retried after a lost response.
/// Identity belongs to this server campaign, not the simulation RNG. Receipts
/// cache only request/result metadata; every response contains CURRENT state.
pub(crate) fn immediate_request(
    g: &mut Game,
    payload: &serde_json::Value,
) -> Result<serde_json::Value, AdvanceError> {
    let me = g.world.player.ok_or("Choose a nation first.")?;
    if payload
        .get("session_id")
        .is_some_and(|v| v.as_str() != Some(&g.session_id))
    {
        return Err(AdvanceError {
            message: "This campaign changed. Review current state before sending orders.".into(),
            requires_review: true,
        });
    }
    let list = payload
        .get("commands")
        .and_then(|v| v.as_array())
        .ok_or("Commands must be a list.")?;
    let fingerprint = serde_json::to_string(list).map_err(|e| e.to_string())?;
    let token = match (payload.get("client_id"), payload.get("request_seq")) {
        (None, None) => None,
        (Some(client), Some(seq)) => {
            let client = client
                .as_str()
                .filter(|c| {
                    !c.is_empty()
                        && c.len() <= 96
                        && c.bytes()
                            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
                })
                .ok_or("Invalid command browser identity.")?;
            let seq = seq
                .as_u64()
                .filter(|s| *s > 0)
                .ok_or("Invalid command sequence.")?;
            if payload.get("session_id").and_then(|v| v.as_str()).is_none() {
                return Err("Protected orders require campaign identity.".into());
            }
            if let Some((last, body, errors)) = g.command_receipts.get(client) {
                if seq < *last {
                    return Err(AdvanceError{message:"A newer order was already processed. Review the current state; older orders will not replay.".into(),requires_review:true});
                }
                if seq == *last {
                    if *body != fingerprint {
                        return Err(
                            "This receipt belongs to a different order. Review the current state."
                                .into(),
                        );
                    }
                    let mut out = state_json(g, None);
                    out["errors"] = serde_json::json!(errors);
                    out["command_replayed"] = true.into();
                    return Ok(out);
                }
            } else if g.command_receipts.len() >= 256 {
                return Err(
                    "Too many command sessions. Continue in an existing browser tab.".into(),
                );
            }
            Some((client.to_owned(), seq))
        }
        _ => return Err("Protected orders need both a browser identity and a sequence.".into()),
    };
    // Parse the complete batch first; malformed input cannot half-commit an
    // immediate command list. Gameplay refusals still report per-order errors.
    advance_commands(&g.world, payload)?;
    let before = g.world.headlines.len();
    let errors = apply_orders(&mut g.world, me, list);
    for headline in g.world.headlines[before..].to_vec() {
        g.record(headline);
    }
    resources::warm(&mut g.world);
    if let Some((client, seq)) = token {
        g.command_receipts
            .insert(client, (seq, fingerprint, errors.clone()));
    }
    let mut out = state_json(g, None);
    out["errors"] = serde_json::json!(errors);
    out["command_replayed"] = false.into();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn diplomatic_receipt_retry_posts_one_cost_and_one_dispatch() {
        let mut g = Game::new(7, Some(NationId::USA));
        let p = serde_json::json!({"session_id":g.session_id,"client_id":"diplomacy-test","request_seq":1,
            "commands":[{"kind":"improve","target":"France"}]});
        let pc = g.world.nation(NationId::USA).political_capital;
        assert_eq!(
            immediate_request(&mut g, &p).unwrap()["errors"],
            serde_json::json!([])
        );
        assert!(g.world.nation(NationId::USA).political_capital < pc);
        let committed = save(&g.world);
        let log = g.log.clone();
        assert_eq!(
            immediate_request(&mut g, &p).unwrap()["command_replayed"],
            true
        );
        assert_eq!(save(&g.world), committed);
        assert_eq!(g.log, log);
        let mut changed = p.clone();
        changed["request_seq"] = 2.into();
        immediate_request(&mut g, &changed).unwrap();
        assert!(immediate_request(&mut g, &p).unwrap_err().requires_review);
    }
    #[test]
    fn malformed_immediate_batch_never_half_commits() {
        let mut g = Game::new(7, Some(NationId::USA));
        let before = save(&g.world);
        let p = serde_json::json!({"session_id":g.session_id,"client_id":"test-client","request_seq":1,
            "commands":[{"kind":"improve","target":"France"},{"kind":"misspelled"}]});
        assert!(!immediate_request(&mut g, &p).unwrap_err().requires_review);
        assert_eq!(save(&g.world), before);
        assert!(g.command_receipts.is_empty());
    }
}
