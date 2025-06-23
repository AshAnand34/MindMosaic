from fastapi import FastAPI
from pydantic import BaseModel
from pipeline import analyze_text

api = FastAPI()

class TextRequest(BaseModel):
    text: str

@api.post("/analyze")
def analyze(request: TextRequest):
    result = analyze_text(request.text)
    return result

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("ml.app.api:api", host="0.0.0.0", port=8000, reload=True)