/* Reading progress only. This model has no storage, clock or campaign commands. */
(function (root, factory) {
  const api = factory();
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.TutorialModel = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";

  const lessons = [
    {
      id: "map-time", title: "Find your bearings", area: "Map & time",
      summary: "Explore your country and understand the clock before making decisions.",
      steps: [
        "Use Home to center your nation, then zoom in and select a province to inspect it.",
        "Read the date and PAUSED or running status. PLAY and the speed buttons run the clock; +1 DAY advances one day.",
        "Keep the game paused while you review these lessons. Marking a lesson complete only records that you have read it."
      ],
      lookFor: "Your nation, a selected province, the current date and the clock status.",
      action: {kind: "home"}, actionLabel: "Explore the map"
    },
    {
      id: "budget-treasury", title: "Give your money a purpose", area: "Economy",
      summary: "Compare your yearly plan with the money the government actually receives and spends.",
      steps: [
        "Open Economy and Your yearly budget. Read ministry funding, the budget estimate and available political capital.",
        "Review Your cash flow for the annual plan and latest daily receipt. An absent receipt means no settlement is recorded yet.",
        "Changing a budget creates a draft. Review it before using Enact & advance 1 day, which applies the decision and moves the calendar."
      ],
      lookFor: "Revenue, spending and balance; whether the budget is a draft, due for renewal or already enacted.",
      action: {kind: "budget"}, actionLabel: "Review the yearly budget"
    },
    {
      id: "construction-effects", title: "Choose useful construction", area: "Construction",
      summary: "Read what a project changes before giving it a place in the funding queue.",
      steps: [
        "Open Suggested construction and read Why this is suggested, or browse Add project.",
        "Review the project's province and national effects, total cost, conditions and estimated time before deciding to build.",
        "Inspect the daily funding limit and queue priorities. You pay for work as it is delivered; unused daily funding is not charged."
      ],
      lookFor: "A clear reason to build, the effects preview and any funding or prerequisite blocker. Higher priorities receive funding first.",
      action: {kind: "construction"}, actionLabel: "Review construction effects"
    },
    {
      id: "industry-operations", title: "Check what industry delivers", area: "Industry",
      summary: "A completed facility needs operating conditions before it can produce useful output.",
      steps: [
        "Open Industry in Economy. Use Needs attention to find facilities reporting a problem.",
        "Compare Recorded output with Recorded operating spending. Awaiting first operation means a receipt is missing, rather than zero production.",
        "Read a facility's reason and available links to review funding, power, resources or research. Supporting facilities explain their role separately."
      ],
      lookFor: "Actual operating receipts and the stated cause of a bottleneck. Financial construction funding does not remove a facility's operating needs.",
      action: {kind: "industry"}, actionLabel: "Inspect industry"
    },
    {
      id: "government-parties", title: "Understand who holds power", area: "Government",
      summary: "Read your leadership, political support and governing decisions together.",
      steps: [
        "Open Government and review Who holds power, then the parties or institutions shown for your political system.",
        "Compare party support, seat share and government role. Support and seats are percentages; a party leader and the national executive can be different people.",
        "Inspect leadership dates and sources, then Review governing decisions. Historical gaps and fictional successors are labelled; campaign succession can depart from history."
      ],
      lookFor: "Who governs now, who supports them and the effects or conditions of a decision before you enact it.",
      action: {kind: "government"}, actionLabel: "Review your government"
    },
    {
      id: "research-components", title: "Research toward a design", area: "Research",
      summary: "Connect discoveries to the components you want your equipment to use.",
      steps: [
        "Open Research and use Research list or the technology map to inspect a discovery and its prerequisites.",
        "Open Equipment designer and its Research tab to compare Chassis, Engines & running gear, Weapons, Armor & protection, Optics & fire control and Communications.",
        "Read each component's availability, required research and tradeoffs. A researched component is an option for a design; it is not a delivered vehicle."
      ],
      lookFor: "Known, available and locked options, and the next prerequisite needed for your intended model.",
      action: {kind: "research"}, actionLabel: "Explore research"
    },
    {
      id: "equipment-procurement", title: "From specification to service", area: "Equipment",
      summary: "Design the model, fund its development and buy equipment that a company has built.",
      steps: [
        "In Designer, compare a platform and its component specifications. Review the model's capability, cost and development requirements before saving or commissioning it.",
        "Use Companies & Procurement to review a manufacturer and development. The manufacturer uses its own working capital and facilities to build stock.",
        "Check Available to buy, review a quantity and purchase cost, then follow delivery into service. Review compatible ammunition supply and fleet needs separately."
      ],
      lookFor: "The separate stages: development, company stock, purchase, delivery and service. A design or fleet target alone does not create vehicles.",
      action: {kind: "equipment"}, actionLabel: "Open the equipment designer"
    },
    {
      id: "diplomacy-military", title: "Read the wider situation", area: "World",
      summary: "Understand conflicts and commitments before deciding how your country responds.",
      steps: [
        "Open World to read conflicts and news, then inspect a country or conflict that matters to your campaign.",
        "Review the costs and conditions of available diplomatic or military decisions before choosing one.",
        "Where force allocation is available, compare deployed force with reserves and sustainable overseas limits. Equipment, access and commitment affect what can deploy."
      ],
      lookFor: "Your current commitments and their constraints. Force points describe an allocation, not a count of soldiers.",
      action: {kind: "world"}, actionLabel: "Review the world situation"
    },
    {
      id: "save-review", title: "Save and review your decisions", area: "Campaigns",
      summary: "Keep a campaign save and compare results when you choose to let time pass.",
      steps: [
        "Use More and Save campaign, or open Campaigns to choose a name under Save your current campaign.",
        "Check that saving succeeds before leaving. Saved campaigns offers Load campaign and Load previous backup.",
        "When you choose to advance time, return to cash flow, construction and industry to inspect the new readings. Tutorial completion is your reading progress, not proof that any game action succeeded."
      ],
      lookFor: "A successful save confirmation and dated results to compare with your plan. Tutorial progress is a browser learning preference, separate from campaign saves.",
      action: {kind: "campaign"}, actionLabel: "Open campaigns"
    }
  ];
  for (const lesson of lessons) {
    Object.freeze(lesson.steps);
    Object.freeze(lesson.action);
    Object.freeze(lesson);
  }
  Object.freeze(lessons);
  const ids = lessons.map(lesson => lesson.id);
  const known = new Set(ids);
  const isId = value => typeof value === "string" && known.has(value);

  function initial(started = false) {
    return {version: 1, done: [], skipped: [], current: ids[0], started};
  }
  function record(value) {
    if (!value || typeof value !== "object" || Array.isArray(value)) return false;
    const proto = Object.getPrototypeOf(value);
    return proto === Object.prototype || proto === null;
  }
  // Stored JSON has data properties. Ignore inherited fields and never invoke getters.
  function own(value, key) {
    const descriptor = Object.getOwnPropertyDescriptor(value, key);
    return descriptor && Object.hasOwn(descriptor, "value") ? descriptor.value : undefined;
  }
  function readIds(value) {
    const result = new Set();
    if (!Array.isArray(value)) return result;
    // Bound recovery work for corrupted, sparse or enormous stored arrays.
    for (let i = 0, length = Math.min(value.length, 256); i < length; i++) {
      const id = own(value, String(i));
      if (isId(id)) result.add(id);
    }
    return result;
  }
  function unresolved(progress, after = -1) {
    for (let offset = 1; offset <= ids.length; offset++) {
      const id = ids[(after + offset) % ids.length];
      if (!progress.done.includes(id) && !progress.skipped.includes(id)) return id;
    }
    return null;
  }

  /** Normalize parsed browser preference data, never a campaign save or achievement. */
  function normalize(raw) {
    try {
      if (!record(raw) || own(raw, "version") !== 1) return initial();
      const done = readIds(own(raw, "done"));
      const skipped = readIds(own(raw, "skipped"));
      const progress = {
        version: 1,
        done: ids.filter(id => done.has(id)),
        skipped: ids.filter(id => skipped.has(id) && !done.has(id)),
        current: null,
        started: own(raw, "started") === true
      };
      const current = own(raw, "current");
      progress.current = isId(current) ? current : unresolved(progress);
      return progress;
    } catch (_) {
      return initial();
    }
  }

  /** Events: start, select(id), complete(id), skip(id), restart. No implicit completion. */
  function advance(raw, event) {
    const progress = normalize(raw);
    let type, id;
    try {
      if (!record(event)) return progress;
      type = own(event, "type");
      id = own(event, "id");
    } catch (_) {
      return progress;
    }
    if (type === "restart") return initial(true);
    if (type === "start") {
      progress.started = true;
      return progress;
    }
    if (!["select", "complete", "skip"].includes(type) || !isId(id)) return progress;
    progress.started = true;
    if (type === "select") {
      progress.current = id; // Completed and skipped lessons remain available for replay.
      return progress;
    }
    if (type === "complete") {
      progress.done = ids.filter(item => item === id || progress.done.includes(item));
      progress.skipped = progress.skipped.filter(item => item !== id);
    } else if (!progress.done.includes(id)) {
      progress.skipped = ids.filter(item => item === id || progress.skipped.includes(item));
    }
    progress.current = unresolved(progress, ids.indexOf(id));
    return progress;
  }
  return Object.freeze({lessons, normalize, advance});
});
