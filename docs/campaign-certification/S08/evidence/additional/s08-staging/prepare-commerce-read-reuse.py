from pathlib import Path
import difflib
import hashlib
import json

base = Path(__file__).resolve().parent
source = base.parent / 'integration/spheres-sim/src/commerce.rs'
original = source.read_text(encoding='utf-8')
text = original

def section(start, end):
    a = original.index(start)
    b = original.index(end, a)
    return original[a:b]

args = 'w, buyer, seller, good, quantity, unit_price_bn, delivery_days'
params = '''
    w: &WorldState,
    buyer: NationId,
    seller: NationId,
    good: Good,
    quantity: f64,
    unit_price_bn: f64,
    delivery_days: u32,
'''

check = section('fn check(\n', 'pub fn quote(\n')
active_start = check.index('    if w.commerce.as_ref().is_some_and(|ledger| {')
active_end = check.index(' {\n        return Err("Finish or cancel', active_start)
active_expr = check[active_start + len('    if '):active_end]
check_body = check[check.index(') -> Result<logistics::RoutePlan, String> {') + len(') -> Result<logistics::RoutePlan, String> {'):]
check_body = check_body.replace(active_expr, 'active_limit.unwrap_or_else(|| active_contract_limit_reached(w, buyer))', 1)
check_replacement = '''// The original ordered contract/cargo predicate. A market query may reuse its
// result for one immutable buyer snapshot; commands still check current state.
fn active_contract_limit_reached(w: &WorldState, buyer: NationId) -> bool {
    ''' + active_expr + '''
}

fn check(''' + params + ''') -> Result<logistics::RoutePlan, String> {
    check_with_active_limit(''' + args + ''', None)
}

fn check_with_active_limit(''' + params + '''    active_limit: Option<bool>,
) -> Result<logistics::RoutePlan, String> {''' + check_body
text = text.replace(check, check_replacement, 1)

quote = section('pub fn quote(\n', 'pub fn proposal_refusal(\n')
quote_body = quote[quote.index('    let available = available_to_sell'):]
quote_replacement = 'pub fn quote(' + params + ''') -> Quote {
    quote_with_active_limit(''' + args + ''', None)
}

fn quote_with_active_limit(''' + params + '''    active_limit: Option<bool>,
) -> Quote {
    let checked = check_with_active_limit(''' + args + ''', active_limit);
    quote_from_checked(''' + args + ''', checked)
}

// Pricing arithmetic and refusal precedence are identical to the original
// quote. Reuse only a check from this same immutable call, never a saved quote.
fn quote_from_checked(''' + params.replace('delivery_days: u32', '_delivery_days: u32') + '''    checked: Result<logistics::RoutePlan, String>,
) -> Quote {
''' + quote_body
text = text.replace(quote, quote_replacement, 1)

refusal = section('pub fn proposal_refusal(\n', 'fn accept_lot(\n')
tail = refusal[refusal.index('    if w.nation(buyer).treasury_bn.unwrap() < q.total_price_bn {'):]
tail = tail.replace('return Some(q.reason);', 'return Err(q.reason);')
tail = tail.replace('return Some("Resolve an existing counteroffer before opening another.".into());', 'return Err("Resolve an existing counteroffer before opening another.".into());')
tail = tail.replace('    None\n}', '    Ok(q)\n}')
refusal_replacement = 'pub fn proposal_refusal(' + params + ''') -> Option<String> {
    proposal_preflight(''' + args + ''', true).err()
}

// The false arm retains the original repeated-check path as a test oracle.
// Both checks precede mutations; standalone accept_lot still revalidates live.
fn proposal_preflight(''' + params + '''    reuse_check: bool,
) -> Result<Quote, String> {
    let checked = check(''' + args + ''')?;
    let q = if reuse_check {
        quote_from_checked(''' + args + ''', Ok(checked))
    } else {
        quote(''' + args + ''')
    };
''' + tail
text = text.replace(refusal, refusal_replacement, 1)

propose = section('pub fn propose(\n', 'pub fn offer_refusal(')
mut_params = params.replace('w: &WorldState', 'w: &mut WorldState')
propose_tail = propose[propose.index('    if q.accepted {'):]
propose_replacement = 'pub fn propose(' + mut_params + ''') -> Result<ProposalResult, String> {
    propose_impl(''' + args + ''', true)
}

fn propose_impl(''' + mut_params + '''    reuse_check: bool,
) -> Result<ProposalResult, String> {
    let q = if reuse_check {
        proposal_preflight(''' + args + ''', true)?
    } else {
        proposal_preflight(''' + args + ''', false)?;
        quote(''' + args + ''')
    };
''' + propose_tail
text = text.replace(propose, propose_replacement, 1)

market = section('pub fn market_quotes(\n', 'pub fn tick_day(')
market_params = '''
    w: &WorldState,
    buyer: NationId,
    good: Good,
    quantity: f64,
    delivery_days: u32,
'''
market_body = market[market.index('    if !active(w)'):]
market_body = market_body.replace('    let cash = w.nation(buyer).treasury_bn.unwrap();', '''    let cash = w.nation(buyer).treasury_bn.unwrap();
    // Each seller sees the same immutable buyer/cargo snapshot. Preserve the
    // original check location and errors while avoiding its nested rescan.
    // Populate lazily: an empty/unsupplied market needs no active-order scan.
    let mut active_limit = None;''', 1)
market_body = market_body.replace('let q = quote(w, buyer, p.nation, good, qty, price, delivery_days);',
    '''let cached_active_limit = if reuse_active_limit {
                Some(*active_limit.get_or_insert_with(|| active_contract_limit_reached(w, buyer)))
            } else { None };
            let q = quote_with_active_limit(w, buyer, p.nation, good, qty, price, delivery_days, cached_active_limit);''', 1)
market_replacement = 'pub fn market_quotes(' + market_params + ''') -> Vec<Quote> {
    market_quotes_impl(w, buyer, good, quantity, delivery_days, true)
}

fn market_quotes_impl(''' + market_params + '''    reuse_active_limit: bool,
) -> Vec<Quote> {
''' + market_body
text = text.replace(market, market_replacement, 1)

tests = r'''
    #[test]
    fn commerce_preflight_reuse_matches_original_refusals_and_full_command_ledgers() {
        let good=Good::Intermediates;
        let price=reference_price_bn(good);
        // Deliberately synthetic boundary states; no earned-campaign claim.
        for state in ["ready","cash","stock","sanctions","disabled","dead","books","ids","sale"] {
            let mut opening=world();
            match state {
                "cash"=>opening.nation_mut(BUYER).treasury_bn=Some(0.0),
                "stock"=>change_stock(&mut opening,SELLER,good,-100.0),
                "sanctions"=>opening.sanctions.push((SELLER,BUYER)),
                "disabled"=>opening.rules.economic_competition=false,
                "dead"=>opening.nation_mut(SELLER).alive=false,
                "books"=>opening.nation_mut(BUYER).treasury_bn=None,
                "ids"=>opening.commerce.as_mut().unwrap().next_id=u64::MAX,
                "sale"=>set_sale(&mut opening,SELLER,good,10.0,1.0,false).unwrap(),
                _=>{},
            }
            let before=crate::save(&opening);
            for (quantity,ask,days) in [(10.0,price,30),(100.0,price,30),(10.0,price*0.5,30),
                (-0.0,price,30),(MIN_LOT,price,30),(f64::NAN,price,30),
                (f64::INFINITY,price,30),(10.0,-0.0,30),(10.0,f64::NAN,30),
                (10.0,f64::INFINITY,30),(10.0,price,0),(10.0,price,366)] {
                let actual=proposal_preflight(&opening,BUYER,SELLER,good,quantity,ask,days,true);
                let expected=proposal_preflight(&opening,BUYER,SELLER,good,quantity,ask,days,false);
                assert_eq!(actual,expected,"{state}: {quantity} / {ask} / {days}");
                if let (Ok(actual),Ok(expected))=(&actual,&expected) {
                    for (a,b) in [(actual.quantity,expected.quantity),(actual.available_quantity,expected.available_quantity),
                        (actual.unit_price_bn,expected.unit_price_bn),(actual.total_price_bn,expected.total_price_bn)] {
                        assert_eq!(a.to_bits(),b.to_bits(),"quote arithmetic must retain exact float bits");
                    }
                }
            }
            assert_eq!(crate::save(&opening),before,"preflight cannot mutate any saved state");
            for (quantity,ask) in [(10.0,price),(100.0,price),(10.0,price*0.5)] {
                let mut actual=opening.clone(); let mut expected=opening.clone();
                assert_eq!(propose_impl(&mut actual,BUYER,SELLER,good,quantity,ask,30,true),
                    propose_impl(&mut expected,BUYER,SELLER,good,quantity,ask,30,false),"{state}");
                assert_eq!(crate::save(&actual),crate::save(&expected),
                    "{state}: exact stocks, escrow, offers, counters, cash, debt and IDs");
            }
        }
    }

    #[test]
    fn commerce_market_read_reuse_matches_original_at_live_contract_and_cargo_limits() {
        let mut opening=world();
        let additional=NationId::Germany;
        opening.nation_mut(additional).treasury_bn=Some(1.0);
        opening.nation_mut(additional).debt_bn=Some(0.0);
        for good in GOODS {
            change_stock(&mut opening,additional,good,100.0);
            set_sale(&mut opening,additional,good,10.0,1.0,true).unwrap();
        }
        let compare=|w:&WorldState| {
            let before=crate::save(w);
            for good in [Good::Intermediates,Good::CapitalGoods,Good::AdvancedComponents] {
                for (quantity,days) in [(-0.0,30),(MIN_LOT,30),(10.0,30),(1000.0,30),
                    (f64::NAN,30),(f64::INFINITY,30),(10.0,0),(10.0,366)] {
                    assert_eq!(market_quotes_impl(w,BUYER,good,quantity,days,true),
                        market_quotes_impl(w,BUYER,good,quantity,days,false),
                        "same ordered quotes for {good:?}, quantity={quantity}, days={days}");
                }
            }
            assert_eq!(crate::save(w),before);
        };
        assert_eq!(market_quotes(&opening,BUYER,Good::Intermediates,1.0,30).len(),2,
            "both consenting sellers must produce actual quotes");
        compare(&opening);
        for _ in 0..MAX_ACTIVE { buy(&mut opening,Good::Intermediates,0.1,30); }
        assert!(active_contract_limit_reached(&opening,BUYER)); compare(&opening);
        settle(&mut opening);
        assert_eq!(opening.commerce.as_ref().unwrap().cargo.len(),MAX_ACTIVE);
        assert!(opening.commerce.as_ref().unwrap().contracts.iter().all(|c|c.remaining_quantity==0.0));
        assert!(active_contract_limit_reached(&opening,BUYER)); compare(&opening);
        // Same contract ID appearing more than once in cargo still consumes one
        // active contract slot; unrelated paid cargo cannot create buyer slots.
        let duplicate=opening.commerce.as_ref().unwrap().cargo[0].clone();
        opening.commerce.as_mut().unwrap().cargo.push(duplicate);
        compare(&opening);
        let due=opening.commerce.as_ref().unwrap().cargo.iter().map(|c|c.due_day).max().unwrap();
        while clock::absolute_day(&opening)<due {next(&mut opening);}
        assert!(!active_contract_limit_reached(&opening,BUYER)); compare(&opening);
    }

    #[test]
    fn commerce_preflight_does_not_replace_live_standalone_acceptance_checks() {
        let mut w=world(); let good=Good::Intermediates; let price=reference_price_bn(good);
        assert!(proposal_preflight(&w,BUYER,SELLER,good,10.0,price,30,true).unwrap().accepted);
        change_stock(&mut w,SELLER,good,-100.0);
        let before=crate::save(&w);
        assert!(accept_lot(&mut w,BUYER,SELLER,good,10.0,price,30).is_err());
        assert_eq!(crate::save(&w),before,"a prior preflight cannot authorize missing stock");
    }
'''
marker = '    #[test]\n    fn manufactured_lot_conserves_cash_stock_and_gdp_through_exact_delivery_once()'
assert text.count(marker) == 1
text = text.replace(marker, tests + '\n' + marker, 1)

(base/'commerce-read-reuse.before.rs').write_text(original,encoding='utf-8',newline='\n')
(base/'commerce-read-reuse.candidate.rs').write_text(text,encoding='utf-8',newline='\n')
patch = ''.join(difflib.unified_diff(original.splitlines(True),text.splitlines(True),
    fromfile='a/spheres-sim/src/commerce.rs',tofile='b/spheres-sim/src/commerce.rs'))
(base/'commerce-read-reuse.patch').write_text(patch,encoding='utf-8',newline='\n')
proof = {'applied':False,'compiled':False,'tested':False,'source':str(source),
    'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
    'patch_sha256':hashlib.sha256(patch.encode()).hexdigest(),
    'candidate_sha256':hashlib.sha256(text.encode()).hexdigest(),
    'scope':'Commerce preflight/check reuse and one immutable market-query buyer active-contract scan; original repeated-read arms retained as test oracles. Standalone accept_lot remains unchanged.'}
(base/'commerce-read-reuse.provenance.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8')
print(json.dumps(proof,indent=2))
