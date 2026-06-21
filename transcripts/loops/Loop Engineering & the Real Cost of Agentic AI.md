# Loop Engineering & the Real Cost of Agentic AI

> **Source:** Workshop/lecture — A skeptical, numbers-driven breakdown of loop engineering hype vs reality, with GitHub's published production case study (62% token savings across 109 production runs), real cost math for three deployment scenarios, and three invisible debts that velocity metrics won't surface.
>
> **Topics Covered:** Paradigm shift, 5 building blocks + 1 most people forget (sub-agent verification), GitHub auto-triage 6 steps, 3 GitHub optimizations (MCP pruning, prefetch, model tiering), prompt caching math (10c vs $3), 25:1 input ratio, model assignment (Haiku/Sonnet/Opus), Strong DM $1k/engineer/day critique, cost-per-commit leading indicator, AI as amplifier not fixer, clean-code wins

---

## Table of Contents

1. [Opening — Loop Engineering Hype vs Reality](#opening--loop-engineering-hype-vs-reality)
2. [The Paradigm Shift](#the-paradigm-shift)
3. [Architecture — The 5 Building Blocks + the 1 Everyone Forgets](#architecture--the-5-building-blocks--the-1-everyone-forgets)
   - [The Five Building Blocks](#the-five-building-blocks)
   - [The Sixth Block: Sub-Agent Verification](#the-sixth-block-sub-agent-verification)
4. [The Concrete Pattern — GitHub's Production Loop](#the-concrete-pattern--gitHubs-production-loop)
   - [GitHub's Auto-Triage 6 Steps](#gitHubs-auto-triage-6-steps)
   - [GitHub's 3 Optimizations](#gitHubs-3-optimizations)
5. [Real Cost Math — Prompt Caching, 25:1 Ratio, Model Tiering](#real-cost-math--prompt-caching-251-ratio-model-tiering)
   - [The 25:1 Input-to-Output Ratio](#the-251-input-to-output-ratio)
   - [Prompt Caching Math](#prompt-caching-math)
   - [Model Tiering](#model-tiering)
6. [Three Budget Scenarios](#three-budget-scenarios)
7. [CFO Quiz — Should You Copy Strong DM](#cfo-quiz--should-you-copy-strong-dm)
8. [The Three Budget Math](#the-three-budget-math)
9. [Quiz — When Velocity Goes Up But Something Broke](#quiz--when-velocity-goes-up-but-something-broke)
10. [The Three Invisible Debts](#the-three-invisible-debts)
11. [Closing and Counterintuitive Insight](#closing-and-counterintuitive-insight)

---

## Opening — Loop Engineering Hype vs Reality

Two things happened recently that sparked this session. First, the speaker made a YouTube video warning engineers about "dark code" — the crisis that AI is generating because AI is actually generating more code than any human can understand. He walked through Dan Shapiro's five levels of AI coding skills, from level zero (write everything by hand) to level five (the "dark factory" where software builds itself and humans are neither needed nor welcome).

Second, an article by Adi Osmany explained how to build the very thing the speaker was warning about — using loop engineering. The speaker was glad Osmany was also skeptical about the paradigm and clarified the risks and requirements.

This session connects two dots: Dan Shapiro told us where we are going in the AI landscape, and Adi Osmany told us how to get there. The speaker's job is to tell you what it actually costs and what issues you need to be aware of — including what it quietly breaks that your metrics will not catch.

> **Key Insight:** Loop engineering is not just hype — it's a real paradigm shift. But it comes with real costs and real risks that the hype often ignores.

---

## The Paradigm Shift

When you are manually prompting any AI coding assistant, you make yourself the bottleneck. Your throughput is capped by your attention and how many agents you can run in parallel in your different terminals.

Loop engineering inverts this concept. It invites you to stop being the person who drives the agent and start being the person who designs the system that drives the agent.

The throughput difference is not 2x, not even 10x. It's the difference between your working hours and your working hours multiplied by however many parallel loops are running while you sleep.

Both Chenny (head of Claude at Anthropic) and Peter Steinberger (author of OpenClaw) have said: "You should not be prompting agents anymore. You should be designing loops that prompt agents."

> **Key Insight:** The shift from "I am the agent's driver" to "I am the system designer" is the core paradigm shift of loop engineering. Your value moves from tactical prompting to strategic system design.

---

## Architecture — The 5 Building Blocks + the 1 Everyone Forgets

### The Five Building Blocks

The five building blocks of a minimal viable loop are:

1. **Automation (Work Trees)** — Git work trees. Use them whenever you have multiple agents and need to leverage agentic loops.
2. **Skills** — Reusable instructions for the agent.
3. **MCP Plugins** — Model Context Protocol integrations for external tools.
4. **Sub-Agents** — Separate agents for separate tasks.
5. **Persistent Memory (State Files)** — What makes the loop get smarter over time rather than just louder.

Think of these five blocks as the organs of a living system. If you leverage all five plus the state file, you have a well-designed loop.

### The Sixth Block: Sub-Agent Verification

The block that most people forget: **sub-agent verification**. When a model writes code and then reviews its own code in the same context window, it already has a position. It will be biased, it will rationalize, and it will not review honestly.

You need a separate agent or sub-agent with zero knowledge of how the code was produced. This is not a nice-to-have — it's a **structural safety requirement**.

Without persistent memory (the sixth block), agents have amnesia. Every run starts with fresh context. Without state files, your expensive loop rediscovers yesterday's work and tries to do it again.

> **Key Insight:** The most-overlooked building block is sub-agent verification. The same agent that wrote the code cannot objectively review it. A fresh sub-agent with zero context is a more honest reviewer.

---

## The Concrete Pattern — GitHub's Production Loop

GitHub — the platform that hosts hundreds of millions of repositories — runs this loop on their own repository. Not in a demo environment, not in a blog post thought experiment. In production.

They achieved 62% token savings across 109 production runs with three optimizations.

### GitHub's Auto-Triage 6 Steps

The six steps GitHub uses:

1. **Trigger** — GitHub Action fires on a schedule
2. **Context Gather** — Two file reads (yesterday's CI failures, open issues, recent commits)
3. **Classification** — One LLM call to classify the issue
4. **Label & Comment** — Two write operations (apply labels, post comments)
5. **State Update** — Log entry in state file

That's the entire loop. Simple, but effective.

### GitHub's 3 Optimizations

Three optimizations that apply to every loop you will ever build:

**1. MCP Tool Pruning**
- Your agent does not need 40 or 100 tools. It probably needs just three.
- Every tool schema in the system prompt is tokens you're paying for on every API call, whether the agent uses the tool or not.
- GitHub had 40 tools and removed 38. That alone saved them 8-12% on input tokens per run.

**2. Prefetch**
- If you know what data you need before the loop starts, fetch it before the loop starts.
- Give the agent a file. Don't make it call a tool for deterministic data.
- Tool calls inside the reasoning loop are expensive. File reads are not.
- Tip: Put text in an MD file instead of taking a screenshot. Screenshots are images that cost far more tokens to parse. Same for PDFs vs MD files.

**3. Model Tiering**
- Using Opus for issue classification is like using a Formula 1 car to get groceries.
- Haiku handles classification at much less cost with no measurable quality loss. GitHub proved this across 109 runs.
- Right-size the model for the task. Implement these three things and you'll spend less than half what you would otherwise.

> **Key Insight:** These three optimizations (tool pruning, prefetch, model tiering) are not GitHub-specific. They apply to every loop you will ever build. Most teams are not using them, leaving 60-70% of their AI budget on the table.

---

## Real Cost Math — Prompt Caching, 25:1 Ratio, Model Tiering

### The 25:1 Input-to-Output Ratio

In a 50-turn agentic coding session, what percentage of total cost comes from input tokens?

The answer: **85% of cost is input. 15% is output.**

The ratio is **25:1 input to output**. The loop reads files (input), generates edits (output), runs bash, rereads modified files (input again), verifies context and instructions (input again).

Every cost optimization must start by reducing what feeds the context, not compressing the output.

Every tool call result, every file read, every bash command output, every error message — all of that flows back as input. In a 50-turn session, the context window keeps accumulating. That's why using `/compact` in Claude Code is a good practice. The more turns you have, the more each turn costs — it's a non-linear curve.

> **Key Insight:** Input tokens dominate costs (85%). If your loop is burning tokens to read deterministic data that you could have fetched in a shell script before the agent even started, you're paying LLM input prices for a file open call. That's an expensive file open call.

### Prompt Caching Math

Without prompt caching, you're paying 3-5x more than you need. In a 50-turn session with 1 million input tokens, you pay for all 1 million at full price.

With caching: cache reads cost 10 cents per million tokens instead of $3. That's a **30x reduction** on your most repeated cost.

For loops specifically, your skills, CLAUDE.md, and Gemini MD are read on every iteration. That's exactly what stable repeated context caching was designed for. Most teams are not using it.

### Model Tiering

Many people default to Opus for everything because it feels premium. Using Opus for issue classification is not premium — it's wasteful.

Haiku handles classification at 3x lower cost with no measurable quality loss. GitHub proved this across 109 runs.

The right model assignment for a three-task loop:
- **Haiku** for classification (cheap, fast, no quality loss)
- **Opus** for hard architecture reasoning (frontier capability where it matters)
- **Sonnet + Batch API** for overnight triage (50% off for async work)

---

## Three Budget Scenarios

Three budget scenarios for a 5-person engineering team:

**Scenario 1: Baseline (No Loop)**
- Each engineer spends $200/day on manual prompting
- Annual cost: $250,000
- Output: Variable quality, human-dependent

**Scenario 2: Basic Loop (No Optimization)**
- Each engineer runs 5 loops/day at $5/loop
- Annual cost: $62,500
- Output: Consistent, but expensive

**Scenario 3: Optimized Loop (GitHub Pattern)**
- Each engineer runs 5 loops/day at $1.50/loop (with caching, tiering, pruning)
- Annual cost: $18,750
- Output: Consistent, validated, cost-effective

> **Key Insight:** The difference between scenario 1 and scenario 3 is 92% cost reduction. Loop engineering isn't just about automation — it's about economics.

---

## CFO Quiz — Should You Copy Strong DM

Strong DM (a company) uses the metric of spending $1,000 in tokens per engineer per day as a productivity benchmark. A CFO reads this and asks: should we be spending that at our company?

**Option A:** Yes, because if not, we fall behind every competitor.
**Option B:** No, because token spent never predicts output.
**Option C:** We need to spend first, then set targets.
**Option D:** Not yet — only rational with full DTO infrastructure.

The correct answer is **D: Not yet.** Token maxing is misleading — people might maximize consumption through bad practices (not caching prompts, using too many MCP servers) without producing better value.

> **Key Insight:** Don't copy Strong DM's $1k/day metric without the infrastructure to validate output. High token spend without behavioral tests equals high velocity with unvalidated output.

---

## The Three Budget Math

If you have $250,000 per year in tokens for a 5-person engineering team, loop engineering is rational **only if** you have:

- The right infrastructure
- The right guardrails
- Enough test coverage (not just unit tests — behavioral tests)
- Digital twins (behavioral clones of Jira, Okta, Slack)
- Thousands of tests running per hour

Without behavioral tests, high token spend equals high velocity with unvalidated output. Strong DM uses digital twin universes — behavioral clones of their systems — to validate that features work under load, in race conditions, and when integrated systems are down.

---

## Quiz — When Velocity Goes Up But Something Broke

Your loop generates code, then you review it in the same Claude session. What is actually happening?

**Option A:** Confirmation bias at machine speed.
**Option B:** Best practice.
**Option C:** Fine if all tests pass.
**Option D:** Industry standard AI review.

The correct answer is **A: Confirmation bias at machine speed.** The same session that wrote the code already has a position. When you ask it to review, it will rationalize. A fresh sub-agent with zero context is a more honest reviewer.

---

## The Three Invisible Debts

Three debts that velocity metrics won't surface:

**1. Technical Debt from AI-Slop**
- AI generates functional but messy code
- Velocity goes up, but maintenance cost increases
- The debt compounds over time

**2. Verification Debt**
- Without sub-agent verification, bugs slip through
- Metrics show "tasks completed" but not "tasks done correctly"
- The cost of undetected bugs far exceeds the cost of verification

**3. Context Debt**
- As loops accumulate context, quality degrades
- Without proper state management, loops redo completed work
- Token costs increase non-linearly with session length

Incident triage is interactive and event-driven, not repeating. Fresh human interrupts beat stale automated context. The loop handles prepared work only — log polling, commit correlation, state file. The human confirms or rejects hypotheses.

> **Key Insight:** Loop engineering does not mean removing the human from everything — it means removing the human from the parts that are repetitive, deterministic, and slow. Context gathering is that part. The judgment is not.

---

## Closing and Counterintuitive Insight

The most counterintuitive insight: **AI is an amplifier, not a fixer.**

If you have clean code, AI will read your patterns and create more clean code. If you have spaghetti code, AI will read those patterns and create more spaghetti code. It will not fix it.

The people who get the best results from loop engineering are those working on green-field projects or clean-code projects. Those working on legacy projects with chaotic, undocumented code will get worse results.

Before deciding whether to use loop engineering, consider the basics: clean code, test coverage, proper documentation, behavioral tests. AI amplifies whatever is already there — good or bad.

> **Key Insight:** The dirty secret of loop engineering: it's not the loop that produces great code — it's the codebase. A great loop on a bad codebase just produces bad code faster. Fix the codebase first, then add the loop.

---

_Structured summary of the workshop transcript. All concepts, examples, and technical details from the original lecture have been preserved and reorganized into a reference-quality article._