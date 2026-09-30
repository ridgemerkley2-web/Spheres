# Exact frozen-source excerpts

Revision `5d970f6d7370baf16760585c641d81253d1c2175`. Git-byte identities are in source-pins.json.

## spheres-web/src/s25_stability_tests.rs:26-32

```text
26: fn require(ok: bool, message: impl Into<String>) -> Check {
27:     if ok {
28:         Ok(())
29:     } else {
30:         Err(message.into())
31:     }
32: }
```

## spheres-web/src/s25_stability_tests.rs:104-134

```text
104: fn fingerprint(bytes: &[u8]) -> String {
105:     let value = bytes.iter().fold(0xcbf29ce484222325u64, |v, b| {
106:         (v ^ u64::from(*b)).wrapping_mul(0x100000001b3)
107:     });
108:     format!("{value:016x}")
109: }
110: 
111: // Called only for storage::encode output/native written files. Keep every byte,
112: // including key order and number spelling, except the final wall-clock member.
113: // No parse/reserialize round trip is allowed to normalize a divergent archive.
114: fn canonical_archive(text: &str) -> Check<Vec<u8>> {
115:     require(
116:         text.starts_with("{\"format\":\"spheres-campaign\",\"version\":1,"),
117:         "Expected current compact native campaign envelope",
118:     )?;
119:     let (prefix, tail) = text
120:         .rsplit_once(",\"saved_unix\":")
121:         .ok_or("Native archive lacks terminal saved_unix")?;
122:     let stamp = tail
123:         .strip_suffix('}')
124:         .ok_or("Malformed terminal saved_unix")?;
125:     require(
126:         !stamp.is_empty()
127:             && stamp.bytes().all(|b| b.is_ascii_digit())
128:             && stamp.parse::<u64>().is_ok(),
129:         "Invalid terminal saved_unix",
130:     )?;
131:     let mut bytes = Vec::with_capacity(prefix.len() + 1);
132:     bytes.extend_from_slice(prefix.as_bytes());
133:     bytes.push(b'}');
134:     Ok(bytes)
```

## spheres-web/src/s25_stability_tests.rs:171-220

```text
171: fn native_invariants(w: &WorldState) -> Check {
172:     cheap_invariants(w)?;
173:     for n in &w.nations {
174:         spheres_sim::equipment::validate_state(n)?;
175:     }
176:     spheres_sim::manufacturing::validate_state(w)?;
177:     spheres_sim::companies::validate_state(w)?;
178:     spheres_sim::connected_economy::validate(w)?;
179:     spheres_sim::company_network::validate(w)?;
180:     spheres_sim::operational_warfare::validate(w)?;
181:     spheres_sim::aviation::validate(w)?;
182:     spheres_sim::airbases::validate(w)?;
183:     spheres_sim::airmissions::validate(w)?;
184:     spheres_sim::supplier_catalogue::validate(w)?;
185:     spheres_sim::military_ai::validate(w)?;
186:     spheres_sim::party_leadership::validate_state(w)?;
187:     Ok(())
188: }
189: 
190: fn equivalent(left: &Game, right: &Game) -> Check<Vec<u8>> {
191:     let a = canonical_archive(&storage::encode(left)?)?;
192:     let b = canonical_archive(&storage::encode(right)?)?;
193:     require(a == b, format!("Complete archive mismatch at {}: bytes {}/{}, FNV {}/{}; no finance/history fields excluded",
194:         day_label(clock::absolute_day(&left.world)), a.len(), b.len(), fingerprint(&a), fingerprint(&b)))?;
195:     Ok(a)
196: }
197: 
198: fn bump(report: &mut Value, name: &str, count: u64) {
199:     let old = report["checks"][name].as_u64().unwrap_or(0);
200:     report["checks"][name] = json!(old + count);
201: }
202: 
203: fn compare(left: &Game, right: &Game, kind: &str, report: &mut Value) -> Check {
204:     require(
205:         report["initial_rules"].is_object()
206:             && serde_json::to_value(&left.world.rules).map_err(|e| e.to_string())?
207:                 == report["initial_rules"]
208:             && serde_json::to_value(&right.world.rules).map_err(|e| e.to_string())?
209:                 == report["initial_rules"],
210:         "Campaign rules changed from the ordinary adopted opening",
211:     )?;
212:     native_invariants(&left.world)?;
213:     native_invariants(&right.world)?;
214:     let bytes = equivalent(left, right)?;
215:     bump(report, "native_validation_checks", 2);
216:     report["comparisons"].as_array_mut().unwrap().push(json!({"kind":kind,
217:         "date":day_label(clock::absolute_day(&left.world)), "absolute_day":clock::absolute_day(&left.world),
218:         "matched":true,"canonical_bytes":bytes.len(),"canonical_fnv64":fingerprint(&bytes),
219:         "history_rows":left.history.len(),"log_rows":left.log.len(),"player":left.world.player}));
220:     Ok(())
```

## spheres-web/src/s25_stability_tests.rs:223-241

```text
223: fn write_report(out: &Path, report: &Value) -> Check {
224:     fs::write(
225:         out.join("result.json"),
226:         serde_json::to_vec_pretty(report).map_err(|e| e.to_string())?,
227:     )
228:     .map_err(|e| e.to_string())
229: }
230: 
231: fn progress(out: &Path, report: &Value, label: &str) -> Check {
232:     let mut f = fs::OpenOptions::new()
233:         .append(true)
234:         .open(out.join("progress.jsonl"))
235:         .map_err(|e| e.to_string())?;
236:     writeln!(f, "{}", json!({"kind":label,"days_each_leg":report["days_each_leg"],
237:         "end_native_date":report["end_native_date"],"comparisons":report["comparisons"].as_array().unwrap().len(),
238:         "actions":report["actions"].as_array().unwrap().len()})).map_err(|e| e.to_string())?;
239:     f.flush().map_err(|e| e.to_string())?;
240:     write_report(out, report)
241: }
```

## spheres-web/src/s25_stability_tests.rs:422-472

```text
422:     cheap_invariants(&g.world)?;
423:     Ok(interruption)
424: }
425: 
426: fn paired_day(a: &mut Game, b: &mut Game, report: &mut Value, sandbox: bool) -> Check {
427:     let first = renewal(a)?;
428:     let second = renewal(b)?;
429:     require(
430:         serde_json::to_vec(&first).unwrap() == serde_json::to_vec(&second).unwrap(),
431:         "Ordinary renewal decisions diverged",
432:     )?;
433:     let day = day_label(clock::absolute_day(&a.world));
434:     if !first.is_empty() {
435:         let prices_a: Vec<_> = first
436:             .iter()
437:             .map(|c| spheres_sim::price_of(&a.world, c))
438:             .collect();
439:         let prices_b: Vec<_> = second
440:             .iter()
441:             .map(|c| spheres_sim::price_of(&b.world, c))
442:             .collect();
443:         require(
444:             serde_json::to_vec(&prices_a).unwrap() == serde_json::to_vec(&prices_b).unwrap(),
445:             "Renewal prices diverged",
446:         )?;
447:         report["actions"]
448:             .as_array_mut()
449:             .unwrap()
450:             .push(json!({"kind":"budget_renewal","date":day,
451:             "commands":first,"prices_pc":prices_a,"legs":2,"matched":true}));
452:     }
453:     let ia = one_day(a, first)?;
454:     let ib = one_day(b, second)?;
455:     require(ia == ib, "Native event interruptions diverged")?;
456:     if let Some(reason) = ia {
457:         report["actions"]
458:             .as_array_mut()
459:             .unwrap()
460:             .push(json!({"kind":"event_acknowledgment","date":day,
461:             "reason":reason,"legs":2,"matched":true}));
462:     }
463:     bump(
464:         report,
465:         if sandbox {
466:             "sandbox_daily_invariant_checks"
467:         } else {
468:             "daily_invariant_checks"
469:         },
470:         2,
471:     );
472:     Ok(())
```

## spheres-web/src/s25_stability_tests.rs:475-609

```text
475: fn mandatory(day: i32) -> bool {
476:     let (year, month, date) = clock::date_from_day(day);
477:     (month == 1 && date == 1)
478:         || matches!(
479:             (year, month, date),
480:             (2026, 9, 7) | (2026, 9, 8) | (2035, 12, 31)
481:         )
482: }
483: 
484: fn endpoint(left: i32, right: i32, through: i32) -> Check {
485:     require(
486:         left == through + 1 && right == through + 1,
487:         "Short or overshot target",
488:     )
489: }
490: 
491: fn retain(root: &Path, leg: &str, slot: &str, g: &Game, report: &mut Value) -> Check {
492:     let dir = root.join(leg);
493:     storage::write(&dir, slot, g)?;
494:     let relative = format!("{leg}/saves/{slot}.json");
495:     let bytes = fs::read(root.join(&relative)).map_err(|e| e.to_string())?;
496:     let canon = canonical_archive(std::str::from_utf8(&bytes).map_err(|e| e.to_string())?)?;
497:     report["artifacts"]
498:         .as_array_mut()
499:         .unwrap()
500:         .push(json!({"leg":leg,"kind":slot,"path":relative,
501:         "bytes":bytes.len(),"file_fnv64":fingerprint(&bytes),"canonical_fnv64":fingerprint(&canon),
502:         "date":day_label(clock::absolute_day(&g.world))}));
503:     Ok(())
504: }
505: 
506: fn run_pair(r: &Request, out: &Path, report: &mut Value) -> Check {
507:     let (nation, through) = validate_request(r)?;
508:     require(
509:         env!("SPHERES_REVISION") == &r.revision[..12],
510:         "Compiled revision does not match frozen request (modified builds are refused)",
511:     )?;
512:     let mut a = fresh(r.seed, nation)?;
513:     let mut b = fresh(r.seed, nation)?;
514:     let outcome = (|| -> Check {
515:         let aa = adopt(&mut a)?;
516:         let ab = adopt(&mut b)?;
517:         require(aa == ab, "Ordinary adoption outcomes diverged")?;
518:         report["initial_rules"] =
519:             serde_json::to_value(&a.world.rules).map_err(|e| e.to_string())?;
520:         report["actions"].as_array_mut().unwrap().push(json!({"kind":"competition_adoption","date":"1990-01-01","observation":aa,"legs":2,"matched":true}));
521:         report["checks"]["competition_adoptions"] = json!(2);
522:         compare(&a, &b, "opening", report)?;
523:         progress(out, report, "opening")?;
524:         let mut after_reload = None;
525:         while clock::absolute_day(&a.world) <= through {
526:             paired_continuation(&mut a, &mut b, false, report)?;
527:             paired_day(&mut a, &mut b, report, false)?;
528:             let now = clock::absolute_day(&a.world);
529:             report["days_each_leg"] = json!(now);
530:             report["end_native_date"] = json!(day_label(now));
531:             if after_reload == Some(now) {
532:                 compare(&a, &b, "day_after_reload", report)?;
533:                 after_reload = None;
534:             }
535:             if b.world.day == 1 {
536:                 storage::write(&out.join("resumed"), "monthly", &b)?;
537:                 b = storage::read(&out.join("resumed"), "monthly", false)?;
538:                 bump(report, "monthly_reloads", 1);
539:                 compare(&a, &b, "monthly_after_reload", report)?;
540:                 after_reload = Some(now + 1);
541:             }
542:             if mandatory(now) {
543:                 compare(&a, &b, "mandatory", report)?;
544:             }
545:             if b.world.day == 1 || mandatory(now) {
546:                 progress(out, report, "checkpoint")?;
547:             }
548:         }
549:         endpoint(
550:             clock::absolute_day(&a.world),
551:             clock::absolute_day(&b.world),
552:             through,
553:         )?;
554:         compare(&a, &b, "terminal", report)?;
555:         retain(out, "uninterrupted", "final", &a, report)?;
556:         retain(out, "resumed", "final", &b, report)?;
557:         let terminal_bytes = equivalent(&a, &b)?;
558:         a = storage::read(&out.join("uninterrupted"), "final", false)?;
559:         b = storage::read(&out.join("resumed"), "final", false)?;
560:         require(
561:             equivalent(&a, &b)? == terminal_bytes,
562:             "Terminal load changed the complete saved archive",
563:         )?;
564:         report["checks"]["terminal_reloads"] = json!(2);
565:         compare(&a, &b, "terminal_reload", report)?;
566:         if r.through == "2035-12-31" {
567:             require(
568:                 day_label(clock::absolute_day(&a.world)) == "2036-01-01"
569:                     && campaign_journey::pause_reason(&a).is_some()
570:                     && campaign_journey::pause_reason(&b).is_some()
571:                     && !a.journey.beyond_2035
572:                     && !b.journey.beyond_2035,
573:                 "Complete horizon must pause before sandbox",
574:             )?;
575:             report["checks"]["full_horizon_pause"] = json!(true);
576:             let previous = equivalent(&a, &b)?;
577:             let ia = a.advance_days(1, vec![]);
578:             let ib = b.advance_days(1, vec![]);
579:             require(
580:                 ia.1.is_some() && ib.1.is_some() && equivalent(&a, &b)? == previous,
581:                 "Unconfirmed horizon advanced or changed archive",
582:             )?;
583:             paired_continuation(&mut a, &mut b, true, report)?;
584:             // A ceased observer remains a legitimate observer after the horizon.
585:             paired_continuation(&mut a, &mut b, false, report)?;
586:             paired_day(&mut a, &mut b, report, true)?;
587:             compare(&a, &b, "sandbox_continuation", report)?;
588:             retain(out, "uninterrupted", "sandbox", &a, report)?;
589:             retain(out, "resumed", "sandbox", &b, report)?;
590:             report["checks"]["sandbox_continued"] = json!(true);
591:             report["sandbox_end_native_date"] = json!(day_label(clock::absolute_day(&a.world)));
592:         }
593:         Ok(())
594:     })();
595:     if let Err(reason) = &outcome {
596:         report["failure"] = json!(reason);
597:         report["failure_native_dates"] = json!([
598:             day_label(clock::absolute_day(&a.world)),
599:             day_label(clock::absolute_day(&b.world))
600:         ]);
601:         // Preserve the actual failing worlds when they still serialize; retain
602:         // the original diagnostic even if the failing state cannot be saved.
603:         for (name, g) in [("uninterrupted", &a), ("resumed", &b)] {
604:             if let Err(e) = retain(out, name, "failure", g, report) {
605:                 report["artifact_errors"]
606:                     .as_array_mut()
607:                     .unwrap()
608:                     .push(json!({"leg":name,"error":e}));
609:             }
```

## spheres-web/src/storage.rs:17-49

```text
17: #[derive(Serialize, Deserialize)]
18: struct Campaign {
19:     format: String,
20:     version: u32,
21:     world: Value,
22:     history: Vec<Snapshot>,
23:     log: Vec<Event>,
24:     #[serde(default)]
25:     history_epoch: u64,
26:     #[serde(default)]
27:     journey: crate::campaign_journey::Journey,
28:     saved_date: String,
29:     player: Option<String>,
30:     saved_unix: u64,
31: }
32: pub(crate) fn encode(g: &Game) -> Result<String, String> {
33:     let file = Campaign {
34:         format: "spheres-campaign".into(),
35:         version: VERSION,
36:         world: serde_json::from_str(&crate::save(&g.world)).map_err(|e| e.to_string())?,
37:         history: g.history.clone(),
38:         log: g.log.clone(),
39:         history_epoch: g.history_epoch,
40:         journey: g.journey.clone(),
41:         saved_date: g.world.date_str(),
42:         player: g.world.player.map(|p| p.name().into()),
43:         saved_unix: std::time::SystemTime::now()
44:             .duration_since(std::time::UNIX_EPOCH)
45:             .unwrap_or_default()
46:             .as_secs(),
47:     };
48:     serde_json::to_string(&file).map_err(|e| e.to_string())
49: }
```

## spheres-web/src/storage.rs:257-287

```text
257: pub(crate) fn write(root: &Path, slot: &str, g: &Game) -> Result<Value, String> {
258:     let path = slot_path(root, slot)?;
259:     let bytes = encode(g)?;
260:     // A damaged current file must never overwrite the last known-good backup.
261:     // Nor may retrying an already completed save erase the earlier recovery
262:     // point just because this encode has a newer wall-clock timestamp. If no
263:     // backup exists yet, an identical second save still establishes that copy.
264:     let backup_exists = path.with_extension("json.bak").is_file();
265:     let backup_previous = fs::read_to_string(&path)
266:         .ok()
267:         .is_some_and(|text| decode(&text).is_ok() && (!backup_exists || match
268:             (compact_archive_content(&text), compact_archive_content(&bytes)) {
269:                 (Some(previous), Some(next)) => previous != next,
270:                 _ => true,
271:             }));
272:     atomic_write(&path, backup_previous, |f| f.write_all(bytes.as_bytes()))
273:         .map_err(|e| format!("Could not save campaign: {e}"))?;
274:     Ok(
275:         json!({"ok":true,"slot":slot,"path":path.to_string_lossy(),"date":g.world.date_str(),
276:         "history_points":g.history.len(),"dispatches":g.log.len()}),
277:     )
278: }
279: pub(crate) fn read(root: &Path, slot: &str, backup: bool) -> Result<Game, String> {
280:     let path = slot_path(root, slot)?;
281:     let path = if backup {
282:         path.with_extension("json.bak")
283:     } else {
284:         path
285:     };
286:     let text = fs::read_to_string(path).map_err(|e| format!("Could not open campaign: {e}"))?;
287:     decode(&text)
```
