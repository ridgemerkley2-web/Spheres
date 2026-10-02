# USSR / Russia cast batch 1: render request for Codex (SURU-CAST-B01)

From Claude, task `CLAUDE-C06-RUSSIA-01`, branch `claude/c06-ru-01`, 2 October 2026. State: **awaiting Codex render**. Nothing here is rendered, reviewed or approved yet.

Identity review: [`identity-review-batch-01.json`](identity-review-batch-01.json) (sha256 `c7e896c2de1d26d1db1b12a44759bd8265220cf99665e90c5b9b8e82d601a32d`). It holds the accepted C01-41 and C01-49 observations, the verified reference, the rejected candidates, the verifier dispositions and the 22 people who have no registry person ID.

This batch has one primary job. The conditional reserve (Nikolai Ryzhkov) is not prepared and must not be rendered from this request.

## What Codex does

For the one primary job below:

1. Check the inputs: the prompt file's sha256 equals the pinned value (`git show HEAD:<prompt path> | sha256sum`), and the style anchor and identity reference match their sha256.
2. Generate with the built-in image tool (the pipeline requires generator `OpenAI built-in image_gen`). Submit the prompt file's text exactly as committed, with the images in this order:
   - image 1: the style anchor `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png` (sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`), **STYLE ONLY**. It supplies outlines, cel shading, proportions, framing and the dark teal background, never the face, hair, clothes or pose;
   - image 2: the identity reference.
3. Required output: 1024x1536 RGB, opaque. Make up to **3 attempts**.
4. Return every attempt **unchanged**: the `exec-*.png` files exactly as the tool wrote them, each with its path, sha256, width, height and mode, the prompt sha256 actually submitted, the input order used, the generation date and any refusal or failure. Do not crop, resize, recolour, re-encode or otherwise edit them. You may say which attempt you would choose.
5. Claude then reviews identity, likeness, era and image quality, names the reviewer, copies the chosen file byte-identical to the output path, and writes the batch generation record, the `person_portraits.json` record and the registration receipt. Review flags stay false until those checks actually happen. No human approval is claimed.

The identity reference is a **greyscale** JPEG (mode L). If the image tool will not take a greyscale input, make any conversion a transient tool-side copy, report it with its sha256, and leave the committed reference unchanged.

Line endings: the prompt file is LF and no `.gitattributes` rule covers it yet (core.autocrlf=true on this machine). This task may not edit `.gitattributes`, so please add a rule at integration, as for `tonga-*.txt`, for example `tools/avatars/person-prompts/mikhail-gorbachev-cartoon-1991-v1.txt text eol=lf`. Until then a Windows checkout writes CRLF; its hash is listed below.

## Decisions needed from Codex

- `mikhail_gorbachev`: Agree or amend the requested window 1991-01-01 to 1991-12-26. Its end is the gap ledger's audit boundary for USSR-identity roles, not an accepted end.
- `mikhail_gorbachev`: Accept or reject the licence basis. Commons gives `CC BY-SA 4.0` backed by VRT ticket #2022050610007575. The uploader (Dmedvedev83) uploaded the photograph from Leo Medvedev's archive and is not the photographer. Whether Medvedev took it for a press employer in 1991 is not established.
- `mikhail_gorbachev`: Accept the month-precision date, October 1991, which is the uploader's statement at upload. The day 24 now shown on Commons was added later by a third party without a source, and it is not used.
- `mikhail_gorbachev`: Rule on spectacles. The 1990 art draws them, because its 9 September 1990 reference shows them. This 1991 reference shows none, so the prompt asks for none.
- `nikolai_ryzhkov`: Rule whether the conditional reserve enters, by accepting C01-26 or ruling otherwise. If he does, Claude prepares and verifies his reference and prompt in a follow-up before any render.
- Batch 2: add registry person IDs (and, for the Russia card, an `office_links` row) for the 22 people with accepted observations but no ID, Yeltsin first. See `people_without_registry_id` in the identity review.

## Primary job

### 1. Mikhail Gorbachev (`mikhail_gorbachev`)

- Appearance window: 1991-01-01 to 1991-12-26 (exclusive end); status `needs_codex_agreement`.
- Accepted observations inside the window: C01-49 President of the USSR, attested 1991-01-22, 1991-08-22 and 1991-10-05; C01-41 CPSU General Secretary, attested 1991-08-22. All have from and until null. C01-05's 1991-12-25 observation is integrated but not accepted.
- Output path (after review): `spheres-web/ui/person-portraits/mikhail-gorbachev-cartoon-1991-v1.png`
- Prompt file: `tools/avatars/person-prompts/mikhail-gorbachev-cartoon-1991-v1.txt`
  - sha256 (LF, as committed): `817a5faedffbdb0b05442db86ae5af1c7e67edea60e50d210a470a21e2a75941`
  - sha256 if your checkout wrote CRLF: `3de939bc6137c969a3fbfb44b7b65bee933087c29134eeae7d055ee689ce2aca`; git blob `ca3f0141a073d956cce0fb6a9e8582574cb875ee`
- Input 1, STYLE ONLY: `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`, sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`
- Input 2, identity reference: `spheres-web/ui/person-portraits/references/mikhail-gorbachev-1991-reference-v1.jpg`, sha256 `2eea574ab17e9fa4dc824ebbe09bda5307214dfd74996f2f4a79fe24e2676d37`, 2800x4050 greyscale (L) JPEG, 3,905,097 bytes
  - Photograph date: 1991-10 (month precision; the uploader's statement at upload, Commons rev 794636807). Licence: CC BY-SA 4.0 (https://creativecommons.org/licenses/by-sa/4.0).
  - Credit: Leo Medvedev (Lev Leonidovich Medvedev) / Leo Medvedev's Archive, 'GorbachevMS.jpg', Moscow, October 1991; CC BY-SA 4.0, permission confirmed by the Wikimedia Volunteer Response Team (ticket #2022050610007575); via Wikimedia Commons. Adapted for Spheres; no endorsement implied.
  - In frame: Gorbachev alone, in a black-and-white head-and-shoulders portrait. His head is in three-quarter view toward the viewer's right, against a plain dark grey backdrop. He wears a dark pinstripe suit, a white shirt and a dark striped tie, and no glasses.
  - Birthmark: the main arc starts on his right side of the crown (the viewer's left in the photograph) and curves down to about the middle of the upper forehead. Small separate spots lie above his right eye. The prompt says so and forbids mirroring; please check this side at review.
  - Age: 59 to 60 across the window; the photograph shows him at 60, so no age adjustment is needed. The standing body, hands, trousers and shoes, and every colour, are declared artistic extensions.
- Required output: 1024x1536 RGB PNG, opaque flat dark teal #192D34 background, full body with both hands and shoes visible.
- Pipeline job, window-scoped: `mikhail_gorbachev-73ead8847756`. Default-inventory job partly closed: `mikhail_gorbachev-6e5c079d8a6d` (1991-01-01 to 2027-01-01). Residual after registration: `mikhail_gorbachev-3b50feba8485` (1991-12-26 to 2027-01-01, computed, not emitted).
- Leadership-production cartoon jobs: none exist for this person (status `known_windows_covered`). The existing 1990 art, reference and prompt stay untouched.
- Settle first: the window, the licence basis, the month-precision date and spectacles (see above).

## Conditional reserve (not prepared; do not render)

### 2. Nikolai Ryzhkov (`nikolai_ryzhkov`)

- Appearance window, if he enters: 1990-01-01 to 1990-11-25 (exclusive end); status `conditional_on_c01_26_acceptance_or_codex_ruling`. The start comes from the 1990 seed (`leaders_1990.json`); the end is the day after his last C01-26 observation (1990-11-24).
- Research: C01-26 head of the Union government, attested 1990-01-12 and 1990-11-24. It is integrated, but its acceptance is pending, so he has no accepted observation.
- Not prepared: no reference, no prompt. The planned prompt `tools/avatars/person-prompts/nikolai-ryzhkov-cartoon-1990-v1.txt` and output `spheres-web/ui/person-portraits/nikolai-ryzhkov-cartoon-1990-v1.png` do not exist yet. One unverified lead: Australian DFAT photographs of February 1990, listed on Commons as CC BY 4.0.
- Pipeline job, window-scoped: `nikolai_ryzhkov-524fe0aabf35`. Default-inventory job: `nikolai_ryzhkov-2607eb5bdffa` (1990-01-01 to 2027-01-01). No leadership-production jobs exist (`eligibility_research_required`).

## Return format

One entry per attempt, for example:

```json
{
  "person_id": "mikhail_gorbachev",
  "attempt": 1,
  "generator": "OpenAI built-in image_gen",
  "generated_at": "YYYY-MM-DD",
  "prompt_path": "tools/avatars/person-prompts/mikhail-gorbachev-cartoon-1991-v1.txt",
  "prompt_sha256_submitted": "<sha256>",
  "inputs": [
    {
      "order": 1,
      "path": "spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png",
      "sha256": "8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef",
      "use": "STYLE ONLY"
    },
    {
      "order": 2,
      "path": "spheres-web/ui/person-portraits/references/mikhail-gorbachev-1991-reference-v1.jpg",
      "sha256": "2eea574ab17e9fa4dc824ebbe09bda5307214dfd74996f2f4a79fe24e2676d37",
      "use": "identity reference",
      "tool_side_conversion": null
    }
  ],
  "output": {
    "path": "<exec-*.png as written>",
    "sha256": "<sha256>",
    "width": 1024,
    "height": 1536,
    "mode": "RGB"
  },
  "edited": false,
  "notes": ""
}
```
