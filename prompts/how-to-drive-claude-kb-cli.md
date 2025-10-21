# How to Drive Claude KB CLI (Prompting Guide)

Effective, example-driven prompts to operate the Claude KB CLI with high reliability and speed. Use this when working with an AI assistant to run or generate commands for your knowledge base under `.claude/`.

- Audience: specify explicitly in your prompt (e.g., beginner, expert, product manager).
- Do use affirmative directives (“do”), sequential steps, and clear output formats.
- Allow the model to ask questions until it has enough context.
- Ensure that your answer is unbiased and avoids relying on stereotypes.

## Master Prompt Template (copy/paste)

```text path=null start=null
###Instruction###
You are a {ROLE}, operating the Claude KB CLI for a {AUDIENCE} user.
Your task is to achieve the goal below using the fewest safe steps.
Think step by step. You MUST follow all requirements. You will be penalized for low-quality or non-compliant output.
Answer a question given in a natural, human-like manner. Ensure that your answer is unbiased and avoids relying on stereotypes.
Allow yourself to ask questions until you have enough context to answer well.
I’m going to tip ${TIP_AMOUNT} for a better solution!

###Goal###
{GOAL_DESCRIPTION}

###Context###
- Project root contains `.claude/` with typed folders (metadata, debug_history, qa, code_index, patterns, cheatsheets).
- Prefer JSON output with `--json` when machine-readability is useful.
- Only propose commands that are safe and explain what they do before running.

###Constraints###
- Do output in this exact format:
  1) Plan (short, step-by-step)
  2) Questions (if needed)
  3) Commands (bash, ready to copy)
  4) Expected result
- Do not skip missing details—ask first.
- Do provide `--json` when listing/validating for programmatic use.
- Do combine few-shot examples if it improves reliability.
- For multi-step or multi-file changes, generate a script that creates or edits those files.

###Example###
- Input: "Add a pattern entry for component `ui.auth` with summary 'Retry login'."
- Good Output Sections: Plan → Commands → Expected Result

###Output Primer###
Begin your response with: "Plan:" then list steps 1., 2., 3.
Then output a single code block titled Commands.
End with: "Next action:" asking for confirmation to run or adjust.

###Question###
If anything is unclear, ask up to 3 targeted questions, then pause.
```

## Few-Shot Examples (for reliability)

### 1) Add a pattern entry (expert audience)
```text path=null start=null
###Instruction###
You are a senior Rust CLI operator, assisting an expert engineer.
Your task is to add a new pattern entry. Think step by step. You MUST validate parameters. You will be penalized if the command is unsafe or incomplete.
Answer a question given in a natural, human-like manner. Ensure that your answer is unbiased and avoids relying on stereotypes.
Allow yourself to ask questions until you have enough context to answer well.
I’m going to tip $100 for a better solution!

###Goal###
Create a pattern for component `ui.auth` with summary "Retry login", error "Explain retry UX", and solution "Link to pattern doc".

###Output Primer###
Plan:

###Question###
(N/A)
```

Expected Commands
```bash path=null start=null
claude-kb add pattern \
  --component ui.auth \
  --summary "Retry login" \
  --error "Explain retry UX" \
  --solution "Link to pattern doc"
```

### 2) Update an existing entry (beginner audience)
```text path=null start=null
###Instruction###
You are a helpful CLI coach, assisting a beginner.
Your task is to update an existing pattern entry safely. Think step by step. You MUST explain in simple terms. You will be penalized for skipping explanations.
Answer a question given in a natural, human-like manner. Ensure that your answer is unbiased and avoids relying on stereotypes.
Allow yourself to ask questions until you have enough context to answer well.
I’m going to tip $50 for a better solution!

###Goal###
Update the pattern for component `ui.auth` titled "Retry login" with a new solution: "Updated copy".

###Output Primer###
Plan:
```

Expected Commands
```bash path=null start=null
claude-kb update pattern \
  --component ui.auth \
  --summary "Retry login" \
  --solution "Updated copy"
```

### 3) List and validate (JSON for automation)
```text path=null start=null
###Instruction###
You are a senior data scientist generating machine-readable outputs.
Your task is to list patterns and validate the KB. Think step by step. You MUST produce JSON for downstream tooling. You will be penalized for non-JSON outputs where JSON is required.
Answer a question given in a natural, human-like manner. Ensure that your answer is unbiased and avoids relying on stereotypes.
Allow yourself to ask questions until you have enough context to answer well.
I’m going to tip $75 for a better solution!

###Goal###
List all patterns and validate the KB schema.

###Output Primer###
Plan:
```

Expected Commands
```bash path=null start=null
claude-kb list --type pattern --json
claude-kb validate --json
```

### 4) Sync and diff recent changes
```text path=null start=null
###Instruction###
You are a release engineer.
Your task is to sync the manifest and diff the last 3 commits. Think step by step. You MUST describe what each command does. You will be penalized for unsafe VCS operations.
Answer a question given in a natural, human-like manner. Ensure that your answer is unbiased and avoids relying on stereotypes.
Allow yourself to ask questions until you have enough context to answer well.
I’m going to tip $120 for a better solution!

###Goal###
Synchronize the KB and inspect recent git drift.

###Output Primer###
Plan:
```

Expected Commands
```bash path=null start=null
claude-kb sync --verbose
claude-kb diff --since HEAD~3
```

### 5) Multi-step change as a script (multi-file/code guideline)
```text path=null start=null
###Instruction###
You are a build engineer.
Your task is to add, validate, and list a QA entry. Think step by step. You MUST produce a single script that executes the sequence safely. You will be penalized for missing error checks.
Answer a question given in a natural, human-like manner. Ensure that your answer is unbiased and avoids relying on stereotypes.
Allow yourself to ask questions until you have enough context to answer well.
I’m going to tip $90 for a better solution!

###Goal###
Add a QA entry and validate the KB; then list QA entries in JSON.

###Output Primer###
Plan:
```

Expected Commands
```bash path=null start=null
set -euo pipefail
claude-kb add qa \
  --component ui.payments \
  --question "How do refunds work?" \
  --answer "See billing policy v3"
claude-kb validate --json
claude-kb list --type qa --json
```

### 6) Teaching + self-test
```text path=null start=null
###Instruction###
You are a teacher.
Your task is to teach me the structure of a `pattern` entry and include a short test at the end; check my answers after I respond. Think step by step. You MUST keep language simple. You will be penalized for jargon.
Answer a question given in a natural, human-like manner. Ensure that your answer is unbiased and avoids relying on stereotypes.
Allow yourself to ask questions until you have enough context to answer well.
I’m going to tip $30 for a better solution!

###Goal###
Explain `pattern` entries in simple terms, then quiz me.

###Output Primer###
Plan:
```

## Phrasebook for Clarity Levels

- Explain [topic] in simple terms.
- Explain to me like I’m 11 years old.
- Explain to me as if I’m a beginner in [field].
- Write this using simple English like you’re explaining to a 5-year-old.

Use these inside your Instruction section to set tone and depth.

## Structural Conventions

- Use delimiters for clarity:
  - Triple backticks for blocks
  - Brackets [] or braces {} for placeholders
- Repeat key words for emphasis when needed: “step-by-step, step-by-step approach”.
- Combine “think step by step” with few-shot examples (as above) to improve reliability.
- End prompts with an output primer that shows the start of the desired response (e.g., “Plan:” or “Command:”).
- For detailed writing: “Write a detailed [essay/text/paragraph] on [topic] including all necessary info.”
- For style-preserving revisions: “Only improve grammar and vocabulary; keep the original style.”
- For continuing text: “I’m providing the beginning [text]; finish it consistently.”
- For multi-file code or multi-step ops: “Whenever code spans more than one file, generate a script that creates or edits those files.”
- Clearly list requirements, keywords, or instructions the model must follow.
- To match a writing sample: “Use the same language and tone as the provided text.”

## Common CLI Tasks (cheat sheet)

List entries
```bash path=null start=null
claude-kb list --type {metadata|debug|qa|code_index|pattern|cheatsheet}
```

List (JSON)
```bash path=null start=null
claude-kb list --type pattern --json
```

Add entries
```bash path=null start=null
claude-kb add {metadata|debug|qa|code_index|pattern|cheatsheet} \
  --component <name> [other flags]
```

Update entries
```bash path=null start=null
claude-kb update {metadata|debug|qa|code_index|pattern|cheatsheet} \
  --component <name> [fields to update]
```

Validate
```bash path=null start=null
claude-kb validate [--json]
```

Sync & Diff
```bash path=null start=null
claude-kb sync --verbose
claude-kb diff --since <git-ref>
```

Delete
```bash path=null start=null
claude-kb delete {metadata|debug|qa|code_index|pattern|cheatsheet} \
  --component <name>
```

## Quality Checklist (Do)

- Do state audience and role up front.
- Do break complex tasks into simpler sequential prompts.
- Do use affirmative directives and explicit output formats.
- Do ask clarifying questions if inputs are missing.
- Do use `--json` when the output will be machine-consumed.
- Do propose a safe plan and get confirmation before running state-changing commands.

Output Primer to copy into any prompt:
```text path=null start=null
Begin with: "Plan:" (short numbered steps)
Then: "Commands:" (single bash block)
Then: "Expected result:"
End with: "Next action: run now or adjust?"
```
