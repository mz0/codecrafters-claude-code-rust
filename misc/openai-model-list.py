#!/bin/env python3

from datetime import datetime, UTC
from fnmatch import fnmatch
import os
import sys
from dotenv import load_dotenv
from openai import OpenAI
"""
openai-model-list.py *4.?*

2026-02-13T16:29:07.017+00:00Z OpenAI Model Names:
* gpt-4.1-2025-04-14
* gpt-4.1
* gpt-4.1-mini-2025-04-14
* gpt-4.1-mini
* gpt-4.1-nano-2025-04-14
* gpt-4.1-nano
"""

load_dotenv()
OPENAI_API_KEY = os.getenv('OPENAI_API_KEY')
if not OPENAI_API_KEY: raise RuntimeError("OPENAI_API_KEY not found")
client = OpenAI()

try:
    pattern = '*' if len(sys.argv) != 2 else sys.argv[1]

    models = client.models.list()  # print("Fetching available OpenAI models...")

    print(f"\n{datetime.now(tz=UTC).isoformat(timespec='milliseconds')}Z OpenAI Model Names:")
    for model in models:
        if fnmatch(model.id, pattern): print(f"* {model.id}")

except Exception as e:
    print(f"An error occurred: {e}")
    print("Please check your API key and ensure it has the correct permissions.")

