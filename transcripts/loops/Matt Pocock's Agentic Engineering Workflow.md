# Matt Pocock's Agentic Engineering Workflow

> **Source:** YouTube — A deep-dive interview with Matt Pocock (TypeScript educator, creator of Total TypeScript) on his agentic engineering setup, harness-first philosophy, AI coding skills, and the future of software development with AI.
>
> **Topics Covered:** Harness-first approach (vs model-first), DX (Developer Experience) vs AX (Agent Experience), skills as procedures vs abilities, the "grill me" adversarial interviewer skill, AFK (away from keyboard) agent workflows, Sand Castle (sandbox agent runner), GitHub Actions integration, sub-agent maker-checker anti-bias pattern, when not to use loops, building businesses with AI, hiring AI-native juniors vs skeptical seniors, the "bitter lesson" in AI coding

---

## Table of Contents

1. [The Harness-First Philosophy](#the-harness-first-philosophy)
2. [DX vs AX — Developer Experience vs Agent Experience](#dx-vs-ax--developer-experience-vs-agent-experience)
3. [Skills as Procedures vs Abilities](#skills-as-procedures-vs-abilities)
   - [The "Grill Me" Skill](#the-grill-me-skill)
   - [Procedures vs Abilities](#procedures-vs-abilities)
   - [Context Window Leakage](#context-window-leakage)
4. [AFK Agent Workflows](#afk-agent-workflows)
   - [Sand Castle — Running Agents in Sandboxes](#sand-castle--running-agents-in-sandboxes)
   - [GitHub Actions Integration](#github-actions-integration)
   - [The Maker + Checker Anti-Bias Pattern](#the-maker--checker-anti-bias-pattern)
5. [Model-First vs Harness-First — The Debate](#model-first-vs-harness-first--the-debate)
   - [The Bitter Lesson](#the-bitter-lesson)
   - [Optimizing the Harness](#optimizing-the-harness)
6. [When NOT to Use Loops](#when-not-to-use-loops)
   - [Incident Triage is Not a Loop](#incident-triage-is-not-a-loop)
   - [The King and Ministers Analogy](#the-king-and-ministers-analogy)
7. [Building Businesses with AI](#building-businesses-with-ai)
8. [Hiring — AI-Native Juniors vs Skeptical Seniors](#hiring--ai-native-juniors-vs-skeptical-seniors)
9. [Practical Action Steps](#practical-action-steps)

---

## The Harness-First Philosophy

**[0:00]** Matt Pocock discusses his approach to AI coding: everyone is obsessed with the model (the "engine of the Formula 1 car"), but they should be more interested in the **harness** — what you can do to get the most out of the system.

**[0:24]** People are focused on the wrong thing — looking at the big shiny new model instead of focusing on what's been working for 30-40 years in software engineering.

**[0:57]** > **Key Insight:** The model is useful, but the harness has an equal amount of work. You have much more control over the harness than you do over the model.

**[1:24]** How do you optimize for token spend? Have a codebase that's easier to make changes in. Then you can employ a stupider model. If your codebase architecture is better, a cheaper model can do the same work because guardrails are better, exploration is easier, and fewer tokens are wasted.

**[1:57]** If you're hamstringing your model from day one with a poor codebase, you'll need a smart model to compensate. But if you build a proper foundation, you can get better results with cheaper models.

---

## DX vs AX — Developer Experience vs Agent Experience

**[2:07]** There's a crucial distinction between DX (Developer Experience) and AX (Agent Experience):

| Concept | Definition | Focus |
|---------|-----------|-------|
| **DX** | Developer Experience | How humans interact with the codebase |
| **AX** | Agent Experience | How AI agents interact with the codebase |

**[2:44]** Anything you can do to improve AX is valuable: better skills, increasing model power, improving the harness, and improving the codebase itself. People often forget that improving the codebase is a key part of improving AX.

**[3:10]** Senior developers are valuable because they know how to build good DX — and there's a huge overlap between good DX and good AX. A codebase that's easy for humans to work with is also easy for AI agents to work with.

**[3:33]** Good architecture fundamentals — the stuff that's been true for 30-40 years — is what makes both DX and AX work well.

---

## Skills as Procedures vs Abilities

**[17:11]** What separates a good agent skill from a bad one? There are two types:

**Procedures** — Skills you intend to run yourself to get the model to behave a certain way. You are in control.

**Abilities** — Skills the model invokes itself, like coding standards. The model pulls them in when needed.

**[17:35]** Matt prefers procedures: "I like to be the one in control. I know my skills, I know my abilities. I don't want to delegate my thinking to the model."

**[18:07]** His most popular skill is **"Grill Me"** — an adversarial interviewer that turns the model into a interviewer asking questions until you reach a shared understanding.

> **Key Insight:** "It's incredibly short — just four or five sentences — and it's unreasonably effective. It turns the agent into an adversarial interviewer asking you questions and popping up with ideas you might not have considered until you reach a shared understanding."

**[18:49]** He uses "Grill Me" as a replacement for plan mode. Before implementing code, you interview the idea to flush out weirdness and unexpected stuff.

**[19:33]** The alternative approach (popularized by OpenAI's "superpowers" skills repo) is to let the model be in control. But Matt prefers personal control.

### The "Grill Me" Skill

The skill is a short procedure that:
1. Turns the model into an adversarial interviewer
2. Asks about your implementation plan
3. Challenges assumptions
4. Pops up ideas you haven't considered
5. Continues until you reach 98% shared understanding

> **Key Insight:** Instead of "one-shotting" an app (describing your vision and saying "build this"), describe your vision and then let the AI interview you about the 10 most consequential decisions. This surfaces problems before you write code.

### Context Window Leakage

**[21:15]** A critical problem with skills: every single skill leaks its description into the context window. If you have 100 skills, you're leaking 100 descriptions — valuable context space.

**[21:41]** Matt's "engineering zoom out" skill has `disable_model invocation: true` — it can only be invoked by the user, so its description is never leaked into context.

> **Key Insight:** Be strategic about what you expose to the AI. Hide most knowledge inside the human and keep the AI's context window for task-specific information only.

---

## AFK Agent Workflows

**[25:14]** Matt does most of his development AFK (Away from the Keyboard) using **Sand Castle**, a tool he built for running agents inside sandboxes.

**[25:40]** Without sandboxes, agents can do weird and dangerous things — randomly delete home directories, exfiltrate environment variables to bad sites, etc. Sand Castle plugs into Docker or Podman to contain agent execution.

**[26:14]** Sand Castle enables:
- Running Claude Code inside sandboxed containers
- Parallelizing multiple agents on one machine
- Using Vercel sandboxes for remote agents
- Pulling commits back into the local workspace

**[26:39]** Combined with GitHub Actions, this creates a powerful automated workflow:
- A PR triggers a GitHub Action
- The action spins up a Review Agent (just a prompt Matt has locally)
- The agent checks the code: typecheck, lint, etc.
- It replies with "cool, it all looks good to me"

**[27:09]** This is "extremely unreasonably effective" because you can parallelize as much as you want without resource constraints on your local machine.

### The Maker + Checker Anti-Bias Pattern

**[30:14]** When building a loop, use **two separate agents**:

1. **Maker agent** — writes the code
2. **Checker agent** — verifies the code with zero context of how it was produced

> **Key Insight:** The same agent that wrote the code cannot objectively review it. This is structural, not optional — it's a safety requirement. A fresh sub-agent with zero knowledge of how the code was produced is a more honest reviewer.

### GitHub Actions Integration

**[48:36]** Matt's setup demonstrates how agents can integrate with existing CI/CD:
- Bug report from live app → create issue → tag as "explore"
- Agent explores and returns structured data: fixable automatically vs needs human
- If fixable: agent implements, reviews, tags for merge
- Human sees the full report (bug + exploration + fix + review decision)
- One-click merge instead of a full debugging session

---

## Model-First vs Harness-First — The Debate

**[27:31]** Matt's hot take: "Everyone is obsessed with the engine of the Formula 1 car, whereas the engine is really only a part of the whole system. You've got the entire chassis, how it moves through the air. Everyone's obsessed with the model and I think they should be more interested in the harness."

**[28:21]** The interviewer challenges: if you swap in a better engine, everything is instantly better, right? Matt agrees but argues for 50/50 thinking — don't over-index on either the model or the harness.

### The Bitter Lesson

**[28:47]** Matt references the "bitter lesson" from ML research: raw compute beats every optimization because compute increases so fast. Maybe optimizing the harness is pointless — just wait for better models.

**[29:49]** His response: he's not waiting for AGI. He's optimizing the harness now. But he acknowledges the tension. His compromise: keep the workspace model-agnostic, apply good software fundamentals that work regardless of model.

**[30:35]** "If I try to overoptimize around a model, I'll lose focus on the fundamentals." His approach: focus on what's been working for 30-40 years in software architecture. That will hold up with the next model.

**[32:02]** People ask how to optimize for token spend? His answer: "Have a codebase that's easier to make changes in." guardrails are better, it's easier to explore, fewer tokens wasted banging against the wall.

> **Key Insight:** Building a proper foundation (good architecture, clear conventions, helpful error messages) is the highest-leverage activity. It makes both cheaper models AND expensive models work better.

---

## When NOT to Use Loops

**[43:10]** In response to the Twitter discourse ("stop prompting your agents, figure out what loops can run forever"), Matt distinguishes between:

**Human in the loop work** — Human and agent talk together. Great for planning, complex implementations, unscoped work.

**AFK work** — You ping off the agent and it does something on its own. Great for prepared, deterministic work.

**[46:06]** Matt mostly thinks of his workflows as **queues, not loops**. Queues, not loops:

**[46:24]** The "queue" is a backlog of tasks that need to be completed:
- Project managers add items to the queue
- The agent picks items off the queue
- Multiple agents can work on different items simultaneously
- There's no single "loop" running forever — work comes in, gets done, comes off

**[47:04]** Thinking about loops as queues better matches how development teams actually work. Multiple developers (agents) pick items, implement them, and the PR merges to resolve the item.

### Incident Triage is Not a Loop

**[48:50]** Incident triage should NOT be fully automated. It's interactive and event-driven, not repeating. Fresh human interrupts beat stale automated context.

**[49:11]** Instead of seeing just a bug report, you see: bug report + exploration + fix + review. It's one click away from merging instead of a whole debugging session.

**[50:05]** How do you remove human-in-the-loop checkpoints? You gain the ability to gate dangerous things from production AND gain insight into your system. Both are valuable.

**[51:47]** You could remove some checkpoints (e.g., "this PR is just an internal refactor, no behavior change"). But then who reviews the AI that's making that determination? You need to sample-check the AI's decisions and improve the system over time.

**[52:35]** Instead of reviewing 20 small fixes, get a custom review summary — an HTML file that knows your common patterns, history of bugs, and learning style. This is more optimized for the AI era than GitHub's PR interface.

---

## Building Businesses with AI

**[54:08]** Matt doesn't think much has changed about building businesses. You still need to:
- Talk to customers
- Figure out what they need
- Build prototypes that solve their actual problem

**[55:21]** AI doesn't give you an advantage in finding product-market fit — that requires real-world conversations. But it gives you a massive leg up in implementation.

**[56:06]** "You cannot be asking the AI to build your app. You need to have the vision. You need to know why you're building it and what problem it's solving."

**[56:21]** You should be asking AI: "What thing can I remove from my app?" Focus on simplicity and UX, not feature accumulation.

---

## Hiring — AI-Native Juniors vs Skeptical Seniors

**[57:15]** How do you reconcile the tension between:
- **Seniors** with 10-20 years experience who get 10x improvement from AI
- **AI-native juniors** who know the tools inside out but lack experience

**[57:47]** Hiring great juniors has always been the goal — enthusiasm beats experience in output. They develop faster and learn faster.

**[58:18]** The key insight: there's a difference between DX and AX. Juniors come at the problem from the AI angle. Seniors come at it from the software fundamentals angle. Both perspectives are valuable.

**[59:38]** "If you have an experimental mindset and you're excited about AI, you're going to get a ton out of it whether you're junior or senior."

**[59:56]** But: "If you're just a tactical programmer just plumbing away doing your work, you're gone. You can't be a code monkey anymore. You need to think strategically."

---

## Practical Action Steps

**[1:00:37]** Matt's advice for the average AI enthusiast:

1. **Delete everything.** Every skill, every plugin, every MCP server. Delete your CLAUDE.md, delete your AGENTS.md. Go back to absolutely nothing.

2. **Observe the agent.** See what it does in its basic mode. Everyone bloats their context window with too much stuff.

3. **Layer on procedures, not abilities.** Add skills that you yourself decide to invoke — not ones the model invokes automatically.

4. **Use his skills repo as a starting point** (github.com/mattpocock/skills). Install them in a way you can customize and experiment.

5. **Delegate implementation to an AFK agent.** "AFK is just an incredible way to work. It takes a little bit of setup, but once it's set up, it just goes crazy."

> **Key Insight:** The harness-first approach means: start with a blank slate, observe the model's raw capabilities, then carefully add only the procedures you control. Keep the human in charge of strategy and let the agent handle tactical implementation.

---

_Structured summary of the interview transcript. All concepts, examples, and technical details from the original conversation have been preserved and reorganized into a reference-quality article._