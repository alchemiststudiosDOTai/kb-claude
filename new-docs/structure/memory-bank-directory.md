---
title: Memory Bank Directory
path: memory-bank/
type: directory
depth: 1
description: Agent-managed workflow tracking for research, planning, and execution logs
seams: [S]
---

# Memory Bank Directory (`memory-bank/`)

## Purpose
A structured, agent-managed system for tracking the complete lifecycle of development work: from initial research through planning to execution. Unlike `.claude/` (which stores categorized knowledge), `memory-bank/` stores timestamped workflow documents organized by phase.

## Organization

### Subdirectories by Workflow Phase

#### `research/` - Investigation and Analysis
Initial research documents analyzing problems, exploring options, and proposing solutions.

**Structure:**
- Timestamped files: `YYYY-MM-DD_HH-MM-SS_description.md`
- Contains: Problem analysis, inconsistencies found, proposed solutions
- Precedes: Planning phase

**Example:**
- `2025-12-03_12-09-03_cli-output-standardization.md`
  - Initial analysis of CLI output inconsistencies
  - Investigation of current behavior
  - Proposed standardization approaches

#### `plan/` - Detailed Implementation Plans
Comprehensive plans with goals, scope, deliverables, milestones, and testing strategies.

**Structure:**
- Timestamped files: `YYYY-MM-DD_HH-MM-SS_description.md`
- Contains: Goals, scope, deliverables, milestones, work breakdown, risks, testing
- Follows: Research phase
- Precedes: Execution phase

**Example:**
- `2025-12-03_12-15-00_cli-output-standardization.md`
  - Goals: Standardize CLI output formatting
  - Scope: All subcommands in `src/cli/`
  - Deliverables: Updated code, tests, documentation
  - Testing strategy: Integration test updates

#### `execute/` - Execution Logs and Audits
Step-by-step records of implementation, commits, and verification.

**Structure:**
- Timestamped files: `YYYY-MM-DD_HH-MM-SS_description.md`
- Contains: Actions taken, commits made, verification results, outcomes
- Follows: Plan phase
- Final phase: Complete workflow trace

**Example:**
- `2025-12-03_12-30-00_cli-output-standardization.md`
  - Step-by-step code changes
  - Commit messages and hashes
  - Test results and verification
  - Final outcome confirmation

## Naming Conventions

### File Naming
- **Format**: `YYYY-MM-DD_HH-MM-SS_description.md`
- **Components**:
  - Date: ISO 8601 date (`2025-12-03`)
  - Time: Hours, minutes, seconds with underscores (`12-09-03`)
  - Description: `kebab-case` topic identifier
- **Uniqueness**: Timestamps ensure chronological uniqueness
- **Traceability**: Matching timestamps across phases link research → plan → execution

### Example Workflow Chain
```
research/2025-12-03_12-09-03_cli-output-standardization.md
   ↓
plan/2025-12-03_12-15-00_cli-output-standardization.md
   ↓
execute/2025-12-03_12-30-00_cli-output-standardization.md
```

## Document Structure

### Research Phase Template
```markdown
# Research: [Topic]

## Problem Statement
[Description of issue or question]

## Investigation
[Findings from codebase exploration]

## Identified Issues
[List of specific problems]

## Proposed Solutions
[Options with trade-offs]

## Recommendation
[Chosen approach with rationale]
```

### Plan Phase Template
```markdown
# Plan: [Topic]

## Goals
[Clear objectives]

## Scope
[What's included and excluded]

## Deliverables
[Tangible outcomes]

## Milestones
[Timeline and checkpoints]

## Work Breakdown
[Detailed task list]

## Risks
[Potential issues and mitigations]

## Testing Strategy
[How success will be verified]
```

### Execution Phase Template
```markdown
# Execution: [Topic]

## Actions Taken
[Step-by-step implementation log]

## Commits
[Commit hashes and messages]

## Code Changes
[Summary of modifications]

## Testing Results
[Test outputs and verification]

## Outcome
[Final status and next steps]
```

## Architectural Significance

### Meta-Knowledge Base
While `.claude/` stores **domain knowledge** (how the CLI works), `memory-bank/` stores **process knowledge** (how the CLI was built). It's a knowledge base about the development process itself.

### Agent Workflow Support
The structure maps directly to agent-driven development:
1. **Research Agent**: Investigates, produces `research/` document
2. **Planning Agent**: Creates detailed plan, produces `plan/` document
3. **Execution Agent**: Implements changes, produces `execute/` document

### Audit Trail
Each feature or change has a complete trace:
- **Why** it was needed (research)
- **How** it was planned (plan)
- **What** was done (execution)

## Relationships

### Relationship to `.claude/`
- **`.claude/`**: Persistent knowledge, categorized by type
- **`memory-bank/`**: Temporary workflow tracking, organized by phase
- **Overlap**: Some workflow documents may be summarized into `.claude/plans/` or `.claude/metadata/`

### Relationship to `src/`
- **`research/`**: Analyzes current code in `src/`
- **`plan/`**: Proposes changes to `src/`
- **`execute/`**: Documents actual changes to `src/`

### Relationship to Git History
- **`memory-bank/`**: High-level workflow narrative
- **Git**: Low-level commit history
- **Together**: Complete story of development (what + why)

## Lifecycle

### Research Phase
1. Problem identified or feature requested
2. Agent explores `src/` and `.claude/`
3. Research document created in `memory-bank/research/`
4. Proposed solutions documented

### Planning Phase
1. Research approved
2. Detailed plan created in `memory-bank/plan/`
3. Goals, scope, deliverables defined
4. Testing strategy outlined

### Execution Phase
1. Plan approved
2. Implementation executed
3. Each step logged in `memory-bank/execute/`
4. Commits referenced and verified
5. Tests run and results recorded

### Completion
1. All phases complete
2. Outcome confirmed
3. Documents may be archived or summarized into `.claude/`
4. Workflow ready for next initiative

## Usage Patterns

### For Major Features
- `research/`: Investigate requirements and approach
- `plan/`: Design implementation with testing strategy
- `execute/`: Track step-by-step development

### For Bug Fixes
- `research/`: Investigate root cause
- `plan/`: Plan minimal fix with regression test
- `execute/`: Verify fix and test addition

### For Refactoring
- `research/`: Identify code smells and improvement areas
- `plan/`: Design refactoring strategy (e.g., Tidy First approach)
- `execute/`: Track incremental changes and verification

## Comparison: `memory-bank/` vs `.claude/`

| Aspect | `memory-bank/` | `.claude/` |
|--------|----------------|------------|
| **Purpose** | Workflow tracking | Knowledge storage |
| **Organization** | Phase-based (research, plan, execute) | Type-based (metadata, patterns, qa, etc.) |
| **Naming** | Timestamped + description | Slugified title |
| **Lifecycle** | Temporary workflow artifact | Persistent knowledge |
| **Audience** | Agent workflow, development process | Developers, users, documentation |
| **Management** | Agent-managed, automated | User-managed, CLI-driven |
| **Structure** | Sequential (research → plan → execute) | Categorical (by entry type) |

## Naming Conventions

### Directory Names
- Singular lowercase: `research`, `plan`, `execute`
- Represents workflow phases in temporal order

### File Names
- Strict timestamp format: `YYYY-MM-DD_HH-MM-SS_description.md`
- Description uses `kebab-case`
- Timestamps sortable chronologically
- Files link across phases via matching descriptions

### Document Titles
- Match file description for consistency
- Format: `# Research: [Topic]`, `# Plan: [Topic]`, `# Execution: [Topic]`
- Clear phase identification

## Extension Points

### Adding New Workflow Phases
Current structure uses three phases, but could extend to:
- `review/` - Code review and feedback
- `deploy/` - Deployment and release notes
- `monitor/` - Post-release monitoring

### Parallel Workstreams
Multiple features can be tracked in parallel:
```
research/feature-a-2025-12-03_12-09-03.md
research/feature-b-2025-12-03_14-15-00.md
plan/feature-a-2025-12-03_12-15-00.md
plan/feature-b-2025-12-03_14-30-00.md
execute/feature-a-2025-12-03_12-30-00.md
execute/feature-b-2025-12-03_15-00-00.md
```

### Archival Strategy
Completed workflows could be:
- Kept in `memory-bank/` indefinitely for audit trail
- Summarized into `.claude/metadata/` as completed work
- Moved to `memory-bank/archive/` for long-term storage
