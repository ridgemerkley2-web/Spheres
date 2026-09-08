# Domestic companies

Companies are fictional game businesses with persistent names, home provinces,
custom vector logos, specialties, advantages, drawbacks, and contract records.
Open **Cabinet → Companies** to compare firms and assign compatible work. New
campaigns include companies; an older campaign offers an explicit **Enable
companies** action. Loading an old save alone does not create a roster.

## Country size and choice

Starting rosters use national GDP and the existing economic sector profile,
not land area. Very large economies receive 3–5 competing firms in each of seven
sectors. Smaller economies receive 2–16 total, weighted toward their developed
sectors. Botswana has three authored fictional firms: Kgale Stoneworks,
Moruwa Minerals, and Kalahari Freight. The US roster includes Continental Works,
Prairie Construction, Summit Engineering, and distinct firms in every sector.

Rapid-delivery specialists are faster but more expensive and may consume more
inputs. Resource specialists can conserve materials at a slower work rate.
Large-contract firms provide more simultaneous slots. No firm has every
advantage. Names, logo identities and local gameplay profiles are checked for
duplicates; new entrants receive persistent identities using the same system.

## Real work and fees

One lead company serves each assignment. A firm can serve only as many
simultaneous assignments as its capacity allows. Assigning or ending a contract
does not mint goods, pay fees, or grant experience. Ordinary operations remain
available without contractors. Ownership, technology, funding, ingredients,
power, storage and freight constraints continue to apply.

| Sector | Assignment | Effect and payment |
| --- | --- | --- |
| Construction | Active building project | More installation work within the shared daily construction budget; contractor fees accompany actual work. |
| Manufacturing | Operating materials/machinery site | Throughput and input-efficiency tradeoffs; fees consume the existing operating appropriation. |
| Research | Research domain with completed centers | More credit from funded, supplied prototype work. Research gates and the prototype contribution ceiling remain in force. |
| Defense | Legacy production line or a custom equipment project | Work-rate and ingredient tradeoffs; paid fees and saved ingredients survive contractor changes and save/load. |
| Energy | Completed generation site | Capacity and fuel-efficiency tradeoffs; fees follow actual dispatched power, allocated through the existing energy funding. |
| Mining | Completed non-oil mine in a province | Additional extraction requires contractor funding from Industry → Minerals & processing. Without funding, ordinary extraction continues. |
| Logistics | Completed mapped freight terminal | Increased local terminal capacity; the government pays a fee on actual handling through normal treasury/debt accounting. Open-sea capacity is unchanged. |

Mining fees use a modeled operating-cost basis equal to 30% of the reference
value of served extraction. Logistics uses a modeled $10 per handled tonne as
its fee basis. Both charge the firm's displayed percentage of that basis.
These are game accounting prices, not historical company quotations.

Bonuses enter the existing physical and fiscal ledgers. There is no additional
GDP award for choosing a company. The directory shows dated work/fee receipts;
an idle contract reports no delivered work. Completed or invalidated work frees
its contract slot while the company's cumulative experience and totals persist.

## Experience and new entrants

Delivered work earns normalized experience, capped at one productive-day unit
per company per day. At 180 XP a firm becomes Experienced; at 540 it becomes
Leading. Each level adds one slot and increases positive specialty bonuses by
25% of their original size. Drawbacks remain. Signing repeated orders cannot
farm experience.

Economic Competition opponents review available contractors monthly through
the ordinary assignment command. They retain existing contracts and make at
most two new assignments per review. The AI never assigns the player's firms.

Growth is checked at month end against an 18-month window. A new entrant needs:

- At least 12% cumulative national GDP expansion and at least 12 growing months.
- At least 12 months of actual activity in its sector.
- At least 10% sector activity growth between the early and late six-month averages.
- An 18-month national cooldown since the previous entrant.

Eligible economies have a deterministic 45% monthly opportunity for one new
specialist. A single GDP spike, idle contracts, and flat sector activity cannot
pass the gate. New firms begin with modest capacity. Identity and growth history
survive saves, and company generation never consumes the simulation's shared RNG.

## Implementation and verification

`spheres-sim/src/companies.rs` owns rosters, eligibility, assignments, experience,
growth, and the read-only snapshot. Existing operating systems settle their own
company effects. `spheres-web/src/companies_view.rs` adds fee explanations;
`companies-ui.js` draws logos and the directory. Commands use the ordinary
session/receipt channel and the server scopes them to the current player.

Verification covers country roster sizes, identity uniqueness, assignment
ownership/capacity, passive behavior, operating constraints, actual fees, growth
gates, contractor switching, saved continuation, stale browser responses,
duplicate-command protection, and distinct SVG identities.
