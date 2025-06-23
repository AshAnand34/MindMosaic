import json
from typing import Dict

# Sentiment Analysis: VADER (for MVP)
from vaderSentiment.vaderSentiment import SentimentIntensityAnalyzer
vader_analyzer = SentimentIntensityAnalyzer()

def analyze_sentiment_vader(text: str) -> str:
    scores = vader_analyzer.polarity_scores(text)
    compound = scores['compound']
    if compound >= 0.05:
        return 'positive'
    elif compound <= -0.05:
        return 'negative'
    else:
        return 'neutral'

# Emotion Detection: CardiffNLP/twitter-roberta-base-emotion
from transformers import AutoTokenizer, AutoModelForSequenceClassification
import torch

# Load model and tokenizer (do this once)
emotion_model_name = 'SamLowe/roberta-base-go_emotions'
emotion_tokenizer = AutoTokenizer.from_pretrained(emotion_model_name)
emotion_model = AutoModelForSequenceClassification.from_pretrained(emotion_model_name)

# Emotion labels for this model
labels = list(emotion_model.config.id2label.values())

def detect_emotions(text, threshold=0.05):
    inputs = emotion_tokenizer(text, return_tensors="pt", truncation=True)
    with torch.no_grad():
        logits = emotion_model(**inputs).logits
    probs = torch.sigmoid(logits).cpu().numpy()[0]  # sigmoid for multi-label
    return [labels[i] for i, p in enumerate(probs) if p > threshold]

# Main pipeline function
def analyze_text(text: str) -> Dict:
    sentiment = analyze_sentiment_vader(text)
    emotions = detect_emotions(text)
    return {
        "sentiment": sentiment,
        "emotions": emotions
    }

# Example usage
if __name__ == "__main__":
    sample = "I am so nervous and scared about the exam."
    result = analyze_text(sample)
    print(json.dumps(result, indent=2)) 