Start by running the following CLI commands to manage your project's knowledge base:

<cli_commands>
<command>
<name>kb-claude cli tool</name>
<description>Create the .claude/ knowledge base structure in your project</description>
<usage>kb-claude init</usage>
</command>

  <command>
    <name>kb-claude new</name>
    <description>Create new knowledge entries with prompted type, tags, and relations</description>
    <usage>kb-claude new "Title"</usage>
    <note>Each entry becomes a typed Markdown file stored in the appropriate .claude/ subfolder (debug_history, patterns, qa, code_index, etc.)</note>
  </command>
  
  <command>
    <name>kb-claude search</name>
    <description>Find existing knowledge and avoid duplication</description>
    <usage>kb-claude search keyword</usage>
  </command>
  
  <command>
    <name>kb-claude validate</name>
    <description>Ensure metadata consistency before commits</description>
    <usage>kb-claude validate --strict</usage>
    <flags>--strict</flags>
  </command>
  
  <command>
    <name>kb-claude manifest</name>
    <description>Generate a summary table of all entries</description>
    <usage>kb-claude manifest</usage>
  </command>
</cli_commands>

.claude/
├── metadata/          # component summaries
├── debug_history/     # debugging timelines
├── qa/               # Q&A and learning notes
├── code_index/       # file or module references
├── patterns/         # reusable fixes or design motifs
├── plans/            # project and release plans
├── other/            # scratch notes ignored by the CLI
├── cheatsheets/      # quick references or how-tos
├── memory_anchors/   # core concepts tracked by UUID
└── manifest.md       # automatically generated summary

Think of it as your project's institutional memory: capture debugging sessions, architecture decisions, and recurring insights as searchable, version-controlled knowledge.

Note: Use `.claude/other/` for ad-hoc notes—the CLI skips anything under that folder.
