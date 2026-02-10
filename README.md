[![progress-banner](https://backend.codecrafters.io/progress/claude-code/f81aa35b-9ee8-40f7-b63b-4d038c011f62)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is (a.t.m. the final) step 5 for Rust solutions to the
["Build Your own Claude Code" Challenge](https://codecrafters.io/challenges/claude-code).

Claude Code is an AI coding assistant that uses Large Language Models (LLMs) to
understand code and perform actions through tool calls. In this challenge,
you'll build your own Claude Code from scratch by implementing an LLM-powered
coding assistant.

Along the way you learn about HTTP RESTful APIs, OpenAI-compatible tool
calling, agent loop, and how to integrate multiple tools into an AI assistant.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

## Steps implemented

* ReadFile tool (steps 1-3)
  ```bash
  ./your_program.sh -p "What is the content of apple.py? Print exact file contents without backticks."
  ```
* WriteFile tool ([step 4](https://app.codecrafters.io/courses/claude-code/stages/oz7))
* Bash tool ([step 5](https://app.codecrafters.io/courses/claude-code/stages/oq5))
  ```bash
  ./your_program.sh -p "Delete the old readme file. Always respond with `Deleted README_old.md`"
  ```
* (extension) `USE_LLM` environment variable (set in [.env](sample.env) and loaded in `your_program.sh`).
  If unset defaults to `anthropic/claude-haiku-4.5`
  ```
  ./your_program.sh -p "Are you ready? Yes/No"
  ```
* (extension) `-m' command line option overrides default and USE_LLM
  ```
  ./your_program.sh -p "Are you ready? Yes/No"  -m nvidia/nemotron-3-nano-30b-a3b
  ```
