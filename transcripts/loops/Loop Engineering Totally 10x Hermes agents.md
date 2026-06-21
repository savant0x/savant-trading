# Loop Engineering Totally 10x Hermes Agents

> **Source:** YouTube — A builder-channel explainer on loop engineering specifically applied to the always-running Hermes agent (and Claude Code). Covers what loop engineering actually is, why it has become newly relevant with the release of Hermes and Fable 5, the two types of loops (deterministic vs non-deterministic), the five-step loop system, the often-overlooked context / feedback / verification / termination / error-handling / state-management design points, and the concrete Hermes-flow that monitors deployed apps, catches production breakages, and fixes them by launching Claude Code non-interactively until tests pass. Also covers an adversarial loop pattern using a different-model verifier (GPT) against a different-model builder (Claude).
>
> **Topics Covered:** Loop engineering as shift from prompt engineering to system design; the "always running" promise of Hermes vs the old manual prompt loop; OpenClaw author and Claude Code creator both calling for loops not prompts; reinforcement-learning analogy; Fable 5 + Opus 4.5 + long-running-task capability shift; why project structure matters as much as the loop; Ralph loop origin and rigid hooks; Claude's slash-goal command and the goal-buddy 2 progression; Hermes / OpenClaw shared philosophy of removing the human from the loop; the five-step loop system (state-check → decide → act → gather feedback → verify-done); why loop engineering handles all five while prompt engineering only handled the decide step; context management (chat context gets buried under tool outputs); feedback quality (tests, screenshots); verification gates; explicit termination conditions; the most-overlooked dimension of error handling; external state files to survive context window limits; deterministic loops (clear done-condition like passing tests); non-deterministic loops (judgment tasks like UI work); AI-slop detector skill; self-evolving skills; adversarial loop pattern (different-model builder + different-model verifier); GitHub PR workflow integration; community AIABS Pro mentioned at the end.

---

## Table of Contents

1. [Opening — Loop Engineering Hype vs the Real Shift](#opening--loop-engineering-hype-vs-the-real-shift)
2. [The Origin Story — Two Builder Communities Converging](#the-origin-story--two-builder-communities-converging)
3. [What Is a Loop — End-Goal, Self-Correction, and the Reinforcement-Learning Analogy](#what-is-a-loop--end-goal-self-correction-and-the-reinforcement-learning-analogy)
4. [The Capability Shift — Opus 4.5, Fable 5, and Long-Running Tasks](#the-capability-shift--opus-45-fable-5-and-long-running-tasks)
5. [Project Structure Matters — Ralph Loop, Hooks, Goal Command, Hermes/OpenClaw](#project-structure-matters--ralph-loop-hooks-goal-command-hermesopenclaw)
6. [The Five-Step Loop System — Built Right](#the-five-step-loop-system--built-right)
7. [Loop Engineering vs Prompt Engineering — Handling All Five Steps](#loop-engineering-vs-prompt-engineering--handling-all-five-steps)
8. [Six Design Points for a Working Loop](#six-design-points-for-a-working-loop)
9. [Deterministic Loops — Tests as the Definition of Done](#deterministic-loops--tests-as-the-definition-of-done)
10. [Non-Deterministic Loops — Judgment Tasks and Adversarial Review](#non-deterministic-loops--judgment-tasks-and-adversarial-review)
11. [Closing](#closing)

---

## Opening — Loop Engineering Hype vs the Real Shift

There's a new term going around and you might have already heard it. It's called **loop engineering**. And just like every other hype term, everyone is talking about it like it's something new. It's not. But when you combine it with an always-running agent like Hermes, it stops being hype.

Most people who are trying to set these up are getting the loop right and missing the thing that actually makes it work. And if you already know there are two types of loops, there's a specific setup inside one of them that almost nobody is doing. Once you see it, the way you think about building with agents changes completely.

By the end of this video, you'll understand exactly what it is, and you'll have it running on Hermes and even Claude Code without you having to step in at all.

With loop engineering, the core idea is simple. You stop being the person who writes the prompt that drives the agent, and instead, you let the agent drive itself. But to see why it's a shift in the first place, you'll have to compare it to what came before. The skill that used to matter was **prompt engineering**, where all our focus went into writing the right series of instructions to drive the coding agent properly.

But loop engineering flips that around. Instead of writing the prompt yourself, you design the system that does the prompt engineering for you and drives the agent on its own. So, the focus moves away from crafting instructions and toward designing systems that run themselves.

---

## The Origin Story — Two Builder Communities Converging

All of this started when the creator of OpenClaw said you shouldn't be prompting your coding agents anymore and that you should focus on designing loops that prompt the agent for you.

And he's not the only one. Boris, the creator of Claude Code, also made the same claim at Anthropic's annual developer conference where he said he doesn't prompt Claude anymore. He's got loops running that prompt Claude and it figures out for itself what needs to be done.

All of it comes down to how well you can set up the systems where you don't have to worry about prompting the agent at all. You define what you need and the agent does the rest. That's exactly where AI-powered development is heading.

---

## What Is a Loop — End-Goal, Self-Correction, and the Reinforcement-Learning Analogy

Before we get into how to actually build them, you need to be clear on what a loop is. A loop is basically a process where you define the end goal and the agent figures out the steps to reach it on its own.

It corrects itself along the way and works around problems until it reaches the goal you set. A few months ago, before models got capable enough to sustain long tasks, this wasn't possible. If you needed to build an app, you'd prompt the agent, monitor what it was doing, check the output yourself, find the issues, and reprompt to fix them. **You were the loop.**

You were the part doing the error checking and course correcting between every step. That's what development still looks like for most people. And that's exactly what loop engineering is about to take off your plate.

This might sound like a brand new concept, but loops have actually been around for a while. **Cron jobs** are a good example of a loop you've probably already seen. They're just tasks scheduled to run repeatedly and automatically without you having to trigger them each time. The only real difference is that a cron job runs at a fixed time. With loops in place, the work stops being about writing the prompt. Your agent's performance on a task comes down to how well you define the end goal.

To some of you, this process will sound a lot like **reinforcement learning**. Reinforcement learning is a way of training a model where you don't show it the right answers — instead, you just tell it when it did well and when it didn't, and it gradually figures out how to get better on its own.

The model finds the right path by trying different things. It gets a positive signal when it moves in the right direction and a negative one when it doesn't. The same idea applies here, except the model itself isn't what's being trained. Instead, the agent is working toward completing the task you want done, iterating on it in the same way a model would improve during training. If it fails the loop you've put on, the agent doesn't mark the task as done — it tries again, keeps going, and corrects itself until it reaches the goal you set.

---

## The Capability Shift — Opus 4.5, Fable 5, and Long-Running Tasks

Your role doesn't shrink — it gets more important because it's your domain knowledge and experience that define the end goal in the first place. And that ends up showing in everything you build and ship. This is exactly why the push toward autonomous loops is only speeding up, and it's showing in every new feature that drops right now.

**Fable 5** is the clearest example yet. Anthropic dropped it even though they'd been calling for a slowdown in AI development because the models are getting capable at a pace that's hard to keep up with. And after releasing it for some time, they even pulled it. They built it for long and complex tasks, and it performs better the longer and more complex the task gets — which is basically the opposite of how models used to work.

This shift really started with **Opus 4.5**. Once that dropped, long-running tasks got dramatically better, and you didn't need to set agents up with carefully guided harnesses anymore — basically, structured setups that walked the agent through each step. The focus moved instead toward preparing the project to run over the long term because the models are now capable enough to handle things on their own without much step-by-step handling.

---

## Project Structure Matters — Ralph Loop, Hooks, Goal Command, Hermes/OpenClaw

The loop isn't the only thing that matters. You also need to structure your project in a way that lets the agent work on its own for a long time without you having to step in.

The **Ralph loop** was one of the first. It worked by setting the end goal and making sure the agent couldn't drift away from it. It did this through **hooks** — scripts that run automatically when something specific happens. This script strictly prevents the agent from marking a task as done unless it had actually met the condition. But hooks are rigid.

So Claude introduced its own **goal command** which did the same thing but with more flexibility. Instead of a hard-coded check, it lets another model decide whether the task is actually finished. They covered **Goal Buddy 2** which built on that by having the agent track its progress in local files and define exactly what "done" looks like before it even starts so it always knows what it's working toward.

The **Hermes agent** and **OpenClaw** were both built on the same philosophy. They take you out of the picture entirely and let the agent handle everything on its own.

---

## The Five-Step Loop System — Built Right

The five-step loop system:

1. **State Check** — Check what state the project is in
2. **Decide** — Model decides what the next action should be
3. **Act** — Agent calls tools, writes files, and runs commands
4. **Gather Feedback** — See what actually happened (test output, screenshots)
5. **Verify Done** — Determine whether the task is complete

This is also where the difference between prompt engineering and loop engineering becomes obvious. With prompt engineering, you're only ever controlling the "decide" step, while loop engineering handles all five together.

---

## Loop Engineering vs Prompt Engineering — Handling All Five Steps

Building a loop that works well means getting a handful of things right. And each one is there because of a specific problem it solves.

---

## Six Design Points for a Working Loop

**1. Context Management**
You pay attention to what goes into the context on every turn because that's what determines what the agent actually knows at any given point. You can't rely on the chat context alone. Even with context windows as large as a million tokens, as the conversation grows, your system prompt and instructions get buried under recent tool outputs. The agent's attention naturally pulls toward whatever is most recent. So the important stuff gets lost.

**2. Feedback Quality**
Feedback is what tells the agent how it did. It can take many forms — the output of a test run, a screenshot of the UI it just built. Whatever form it takes, that's what the agent reads to figure out its next move.

**3. Verification Gates**
Verification gates turn feedback into a clear verdict. They're the checkpoints that tell the agent whether a task is actually done or not.

**4. Termination Conditions**
A rule that tells the loop when to stop. This has to be set explicitly — otherwise, the agent either quits too early or keeps going without making real progress.

**5. Error Handling**
The thing people most often overlook. You have to spell out what the model should do when a tool call fails, so the system handles it cleanly instead of leaving things in a broken state that creates more problems.

**6. State Management**
Keep track of where the task is as the conversation grows. The context window can't hold everything forever, so you lean on external files that track information for the agent and let it keep working without losing the thing.

> **Key Insight:** Since you're handing the job of figuring out the path over to the model, loops get expensive in tokens. The more tokens a loop can work with, the better it tends to handle the task. So you need to be deliberate about when you actually use them.

---

## Deterministic Loops — Tests as the Definition of Done

There are two types of loops. The first is called the **deterministic loop**. You use it for tasks that have a clear definition of what "done" actually looks like — tests passing, code compiling successfully, or anything like that. These loops are fairly straightforward to work toward because the end goal is clear. The model knows exactly what it needs to do before it can call the task done.

Since **Hermes** is always running, it's a really good agent to implement this loop on. The core of a deterministic loop is the clear definition of the end goal, and for apps you've deployed, that definition is your tests. So you can point the Hermes agent at any app you've deployed with test cases and have it monitor it for you.

If a change or a commit ends up breaking production, you can set up an automation on Hermes to catch it. The reason it works best here is that it comes with the **self-evolving skills** feature — it automatically creates and evolves skills based on the workflow, which keeps the health of the app in check.

Once you've set up that monitoring automation, you can ask it to launch Claude Code in **non-interactive mode** — running it on its own without you having to drive it — and have it fix issues in a loop until all the test cases pass. It loads skills like the sub-agent-driven development skill and the GitHub PR workflow skill which tell it how to manage the app on GitHub. It first identifies the issues that were breaking production, then launches Claude Code in non-interactive mode which takes the tests and commits the changes once all of them pass.

After it has run every test and fixed whatever was causing production to fail, it uses the GitHub CLI to commit the changes. The app ends up running without any failures because it has confirmed that all the checks for a successful deployment are in place.

---

## Non-Deterministic Loops — Judgment Tasks and Adversarial Review

The second type is the **non-deterministic loop**. These are tasks where you can't just set a clear rule to check whether the job is done the way you can with deterministic loops. Because of that, there's no clean way to verify the outcome. These are the kinds of things that we as humans can look at and judge for ourselves — like building a UI or implementing a feature that needs a judgment call.

When you're working with a non-deterministic loop, the workflow is different. If you're applying AI to UI, you already know that it tends to fall back to the same patterns all the time. That's why they created a skill called **AI-slop detector**, which holds all the instructions on how to avoid AI slop and lists the patterns that actually give it away.

The reason they're using Hermes again is the self-evolving skills. If they still find AI slop in the UI after running the skill, the skill can update itself to incorporate that feedback directly. So they asked Hermes to use the skill and check whether the UI has any of those patterns. If it does, it fixes them and launches Claude Code in non-interactive mode to run the skill and keep fixing what it finds until there's nothing left to fix.

Another benefit of Hermes is that the model reviewing the work is different from the one building it. They were using the GPT models which are known to be among the best for code review. So the Claude models become the builder and the other agent becomes the verifier. That's what completes the **adversarial loop** where the two check each other's work.

Once that loop ran, it generated a much better UI than the generic output the Opus models are putting out nowadays. And if you still spot any sign of AI slop in the UI after the agent loop has ended, you can just mention it and it will update the skill for you, strengthening the verifier you already have.

They've enhanced this skill to match multiple AI slop patterns that they and Hermes identified collectively. If you want to use this skill, you can get it from their community, **AIABS Pro**.

---

## Closing

The key takeaways from this session:

1. Loop engineering is a shift from "writing prompts" to "designing systems that prompt themselves"
2. The five-step loop system (state-check → decide → act → gather feedback → verify-done) handles what prompt engineering only addressed in one step
3. Six design points matter: context management, feedback quality, verification gates, termination conditions, error handling, and state management
4. Deterministic loops work for tasks with clear done-conditions (tests passing)
5. Non-deterministic loops need adversarial review (different-model builder + different-model verifier)
6. Hermes and OpenClaw are built on the same philosophy: remove the human from the loop entirely
7. Self-evolving skills are what make the loop actually work — they prevent drift and quality degradation

> **Key Insight:** The loop isn't the only thing that matters. You also need to structure your project in a way that lets the agent work on its own for a long time. The Ralph loop, Claude's goal command, Hermes, and OpenClaw all share the same philosophy: you define the end goal and the agent figures out the rest.

---

_Structured summary of the workshop transcript. All concepts, examples, and technical details from the original lecture have been preserved and reorganized into a reference-quality article._