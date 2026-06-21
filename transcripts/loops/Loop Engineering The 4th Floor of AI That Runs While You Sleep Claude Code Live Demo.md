# Loop Engineering: The 4th Floor of AI That Runs While You Sleep (Claude Code Live Demo)

> **Source:** Builder community workshop live session — Loop engineering as the stage beyond prompt/context/harness engineering, with a Claude Code live demo of an autonomous build/test/fix loop overnight on a website.
>
> **Topics Covered:** Four floors of leverage (prompt → context → harness → loop), four-condition loop test (deterministic + clear-exit + empirical-verifier + stable methodology), loop vs deterministic orchestration, minimal viable loop anatomy (automation + goal + skill + state + gate), worktree isolation + parallel agents, sub-agent maker-checker anti-bias, Playwright UI verification, Ralph Wiggum loop criticism, when NOT to use a loop, non-engineering loops, anti-patterns

---

## Table of Contents

1. [Why This Session — From Prompt Engineering to Loop Engineering](#why-this-session--from-prompt-engineering-to-loop-engineering)
2. [The Four Floors of Leverage](#the-four-floors-of-leverage)
   - [Floor 1: Prompt Engineering](#floor-1-prompt-engineering)
   - [Floor 2: Context Engineering](#floor-2-context-engineering)
   - [Floor 3: Harness Engineering](#floor-3-harness-engineering)
   - [Floor 4: Loop Engineering](#floor-4-loop-engineering)
3. [When You Actually Need a Loop — The Four-Condition Test](#when-you-actually-need-a-loop--the-four-condition-test)
4. [Loop vs Deterministic Orchestration](#loop-vs-deterministic-orchestration)
5. [The Five Building Blocks of a Minimal Viable Loop](#the-five-building-blocks-of-a-minimal-viable-loop)
6. [Workflow Setup](#workflow-setup)
7. [Sub-Agent Maker + Checker — Anti-Bias Review](#sub-agent-maker--checker--anti-bias-review)
8. [Live Demo — Fixing a Website Overnight](#live-demo--fixing-a-website-overnight)
9. [The Hard Gate That Cannot Fail](#the-hard-gate-that-cannot-fail)
10. [When NOT to Use a Loop — Six Misfit Domains](#when-not-to-use-a-loop--six-misfit-domains)
11. [Loops Outside Engineering](#loops-outside-engineering)
12. [The Loop Pattern Will Eat Itself](#the-loop-pattern-will-eat-itself)
13. [Anti-Patterns — When Loops Go Wrong](#anti-patterns--when-loops-go-wrong)

---

## Why This Session — From Prompt Engineering to Loop Engineering

This session is part of a weekly builder community workshop series where the group picks one topic, discusses it, and explores how to integrate it into their current stack. The subjects are always challenging, and the environment is relaxed and hands-on.

The group has been through a journey: from prompt engineering to context engineering, and now to harness engineering and loop engineering. Each era has pushed the builder further from the keyboard — from being the person who drives the agent, to designing the system that drives the agent.

Loop engineering is the newest paradigm. The team at Anthropic reported being able to merge eight times more code per day than they could two years ago. Even though Anthropic called this an "overstatement," the numbers point to a real productivity gain.

> **Key Insight:** The loop doesn't replace the first three floors — it automates them. You can't design a loop that you can't run by hand first. Everything from floors one through three is tied into loop engineering.

---

## The Four Floors of Leverage

There are four floors of leverage in AI coding, each representing a shift in where the human's leverage lives:

### Floor 1: Prompt Engineering

The era of **words**. The skill was phrasing — how best to ask the AI to do what you wanted. You held the agent through every turn, and your leverage was in how you structured your prompts.

Techniques evolved from zero-shot examples to one-shot, two-shot, three-shot, and chain-of-thought. But ultimately, it all worked with words. You were in the chair the whole time.

### Floor 2: Context Engineering

The era of **inputs**. The realization hit: the model is only as good as what's in its window. People started optimizing what the model sees — the right files, memory, connectors, skills, MCPs.

This was the era of "context rot" — the more you feed tokens into the window, the less efficient the output becomes. The skill became **curation**: choosing what the model gets to see before passing it over.

The ICM (Interpretable Context Methodology) framework was crucial here — the whole point was to avoid overloading the context with irrelevant information.

### Floor 3: Harness Engineering

The era of **runtime**. Giving the model tools, the ability to run bash commands, search the internet, read its own output, see what broke, and figure out how to fix it.

Claude Code, Codex, and similar tools are all harnesses. The loop exists now, but you're still standing next to it, ready to interrupt when something happens or steer it in the right direction.

The harness gave agents the ability to loop. But with loop engineering, it takes you out of the loop entirely.

### Floor 4: Loop Engineering

The era of **systems**. For the first time, the person who used to prompt is now being replaced by a system that does the prompting for you while you're off the desk.

The system finds the work, checks it, and describes the next move. It goes through iterations on its own. You design it once, and the system prompts the agent from then on.

Loop engineering allows you to delegate tasks to agents, and the agent runs the prompting, the checking, and the decision-making. It tries to ensure it does what it's supposed to do — autonomously.

> **Key Insight:** Each era pushed the builder further from the keyboard. Prompt engineering = hand on the keys. Context engineering = curating inputs. Harness engineering = standing beside the loop. Loop engineering = out of the chair entirely.

---

## When You Actually Need a Loop — The Four-Condition Test

Do you actually need a loop? Not everything needs one. The workshop uses a **four-condition test** to determine whether a loop is appropriate. If you miss even one condition, keep it as manual prompting.

**Condition 1: The task repeats itself.** Are you dealing with a task that repeats over and over, like a skill? If the task is repeatable, it's worth a loop.

**Condition 2: Verification can be automated.** Can the verification be automated? If yes, does the budget absorb the waste? Loops run multiple iterations, so the budget must justify the token cost.

**Condition 3: Budget justifies it.** The budget must be willing to absorb the waste of multiple iterations. If the budget satisfies the use case, a loop is valid.

**Condition 4: The agent has senior engineer tools.** Is the agent given real tools — the ability to run code, search the web, read output, and fix errors? If the agent has senior engineer tools, it deserves a loop.

All four conditions must be passed. If you miss one, keep it as manual prompting.

**Who should NOT use a loop:**
- Solo builders on consumer plans (build lands before payoff)
- Teams without automated verification
- Teams whose bottleneck is review, not typing
- One-off exploratory or judgment call work

**Who should use a loop:**
- People doing repetitive, machine-checkable work
- People with budget to run loops
- Teams with real test coverage
- Teams already running multiple agents

> **Key Insight:** Loop engineering is real, but most people don't need it yet. If you're a solo builder on a consumer plan, this might not be for you. But when you reach the point where the four conditions are met, the payoff is significant.

---

## Loop vs Deterministic Orchestration

A question from the audience: How does loop engineering relate to workflow orchestration with multiple stages and crossroads?

The key distinction:

| Concept | Description |
|---------|-------------|
| **Single Loop (MTL)** | One loop that answers: "Is this task done?" Mastering the gates. |
| **Orchestrator** | A loop of loops. Composes multiple atoms into a directed workflow. Mastering routing between gates. |

Loop engineering is mastering the gates (is this task done?). Orchestration is mastering the routing between gates (what's the next task given everything that's happened so far?).

An orchestrator is what you get when you compose many minimal viable loops into a directed workflow where the output of one loop becomes the trigger or input for the next.

---

## The Five Building Blocks of a Minimal Viable Loop

A minimal viable loop (MTL) has five building blocks:

1. **Automation** — The trigger (cron, GitHub action, schedule)
2. **Goal** — What "done" looks like (the condition to check)
3. **Skill** — The instructions for how to do the work
4. **State** — Persistent memory between iterations (files, JSON, MD)
5. **Gate** — The check that determines whether to continue or stop

The loop doesn't replace the first three floors — it automates them. You design it once, and the system runs it forever.

---

## Workflow Setup

The workshop transitions to a live demo. The presenter opens Warp (a terminal-based coding tool) and pastes in a prompt to start the loop.

The demo: a website has been built, and the audience is asked if they should ship it. The answer is no — the website has issues. The presenter will use the loop to fix them overnight.

---

## Sub-Agent Maker + Checker — Anti-Bias Review

Before the demo, the presenter runs `npm run gates` — a command that checks the website against the spec. Out of 7 checks, only 3 pass. The website is not ready to be shipped.

Failures include:
- Missing responsive viewports
- Only 2 of 3 required feature cards found
- All images are empty

The presenter opens Claude Code and pastes the spec document. The loop will run continually with a `/goal`: run the gates command, ensure all spec.md rows are green, update state.md, and open a draft PR when the gate is green.

As a prompt engineer, the alternative would be to manually go through each failure, fix it, get a response, go to the next one — sitting at the computer the whole time. With the loop, the agent does this autonomously.

> **Key Insight:** The sub-agent maker + checker pattern uses two agents: one to make changes, another with zero context of how the code was produced to verify. This prevents confirmation bias — the same agent that wrote the code can't objectively review it.

---

## Live Demo — Fixing a Website Overnight

The loop starts. Claude Code reads the spec document and begins working on the failures. The agent:

1. Reads the spec
2. Investigates the root cause of each failure
3. Fixes the code
4. Runs the gates again
5. Checks if all tests pass

The goal is to run the command, ensure all spec.md rows are green, update state.md, and open a draft PR when the gate is green.

The presenter explains the `/loop` and `/goal` commands in Claude Code — these are what close the loop. The agent works, evaluates, and if the goal isn't achieved, it runs back in the loop.

The agent is working on the website now. It's reading the spec, identifying issues, and fixing them. The presenter walks through what's happening in the background.

The agent has completed the fixes. The gates now pass. The website is ready to ship.

The presenter shows the GitHub repository that was created — the agent automatically took screenshots, added them to the README, and pushed everything to GitHub.

The result: a professional-looking repository with screenshots, documentation, and working code — all created autonomously.

> **Key Insight:** This is the entire SDLC pipeline — build, test, fix, document, push — solved end-to-end by the loop. The human only defined the spec and reviewed the final output.

---

## The Hard Gate That Cannot Fail

The most critical part of the loop is the **gate** — the check that determines whether the task is actually done. This gate cannot fail.

The gate is `npm run gates` — a command that checks the website against the spec. If all rows are green, the task is done. If not, the loop continues.

The gate is what separates loop engineering from simple automation. Without a reliable gate, the loop has no way to know when to stop.

The presenter emphasizes: the gate must be **empirical** — it must check actual behavior, not just assume the code is correct. The gates command opens the browser, takes screenshots, and verifies the UI matches the spec.

---

## When NOT to Use a Loop — Six Misfit Domains

Loops are not for everything. Six domains where loops are a poor fit:

1. **Exploration** — When you don't know what to build (if you knew, you'd put it in the PRD)
2. **One-off tasks** — Single executions that don't justify the setup cost
3. **Judgment work** — Subjective decisions that require human intuition
4. **Creative work** — Tasks that require taste, style, or artistic judgment
5. **Unscoped work** — Tasks without clear acceptance criteria
6. **Emergency response** — Incident triage that requires human-driven, real-time decisions

Incident triage is interactive and event-driven, not repeating. The fresh human interrupt sending updates beats stale automated context. The loop handles prepared work only — log polling, commit correlation, state file updates. The human confirms or rejects hypotheses.

> **Key Insight:** Loop engineering does not mean removing the human from everything — it means removing the human from the parts that are repetitive, deterministic, and slow. Context gathering is that part. The judgment is not.

---

## Loops Outside Engineering

Loops are not just for software engineering. They can be applied to:

- **Content generation** — Write, review, publish loops
- **Research** — Gather, synthesize, verify loops
- **Customer support** — Triage, respond, escalate loops
- **Data processing** — Extract, transform, validate loops

Any task that repeats, has clear acceptance criteria, and can be verified automatically is a candidate for a loop.

---

## The Loop Pattern Will Eat Itself

A warning: the loop pattern is powerful but can be overapplied. If everything is a loop, you end up with loops managing loops — meta-loops that consume resources without producing value.

The key is to use loops only where they provide clear value: repetitive, testable, deterministic work with clear exit conditions.

---

## Anti-Patterns — When Loops Go Wrong

Common anti-patterns:

1. **Infinite loops** — No exit condition, or the gate never passes
2. **Amnesia loops** — State files not properly maintained, loop redoes completed work
3. **Cascading failure loops** — One failure triggers another, loop never recovers
4. **Over-optimization loops** — Loop spends more tokens than it saves
5. **False positive gates** — Gate passes but the task isn't actually done

The presenter shares a story: a loop was set up to monitor Kubernetes pods. It worked perfectly for 3 days, then started crashing because the state file grew too large. The lesson: state management is critical.

Another anti-pattern: using loops for tasks that require human judgment. A loop can check if code compiles, but it can't tell if the code is the right solution to the problem.

> **Key Insight:** The loop is a tool, not a religion. Use it where it provides value, not everywhere. The four-condition test exists for a reason — respect it.

---

_Structured summary of the workshop transcript. All concepts, examples, and technical details from the original lecture have been preserved and reorganized into a reference-quality article._