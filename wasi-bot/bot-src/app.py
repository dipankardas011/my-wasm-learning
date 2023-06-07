from fastapi import FastAPI
from fastapi.responses import RedirectResponse
import requests

from transformers import pipeline, set_seed
generator = pipeline('text-generation', model='gpt2-medium')
set_seed(42)


app = FastAPI()

@app.get("/")
async def docs_redirect():
    return RedirectResponse(url='/docs')

@app.get("/generate")
def generate(text: str):
    output: list[dict] = generator(text, max_length=200, num_return_sequences=1)
    print(output)
    return output[0]["generated_text"]
