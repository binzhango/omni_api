# omni_api

`omni_api` is an API payload transformation agent. It accepts input payloads from one format and converts them into the schema required by a target backend API.

This project is designed for systems that integrate with multiple providers, services, or legacy endpoints that all expect different request structures.

## Project Introduction

In real-world integrations, clients often send data in shapes that do not match backend contracts. `omni_api` solves this mismatch by acting as a normalization layer between producers and consumers.

The agent can:
- analyze incoming payload fields
- map source keys to backend-required keys
- extract nested values into flat backend fields
- keep required fields and drop irrelevant fields
- output a clean payload that backend APIs can consume directly

This reduces coupling between clients and backend services, simplifies integration logic, and makes backend contracts easier to enforce.

## Personal Requirements

This project follows these implementation requirements:
- Rust-first core implementation for high-performance transformation and routing logic
- Python wrapper around the Rust core so it can be integrated easily into Python ecosystems
- Middleware-oriented architecture to keep request latency low while standardizing payloads
- RESTful API design defined with OpenAPI
- FastAPI as the Python API framework

## Example

Target backend API expects:

```json
{
  "name": "John Doe",
  "age": 30,
  "email": "john@example.com"
}
```

Incoming payload from client:

```json
{
  "full_name": "John Doe",
  "age": 30,
  "contact": {
    "email": "john@example.com"
  },
  "extra_data": "not needed"
}
```

`omni_api` transforms it into:

```json
{
  "name": "John Doe",
  "age": 30,
  "email": "john@example.com"
}
```

## Where It Helps

- Integrating multiple upstream clients with one backend contract
- Bridging old and new API formats during migration
- Standardizing payloads before validation and business logic
- Reducing custom transformation code in each service
- Normalizing request/response formats across multiple LLM providers

## Model Provider Targets

The agent is designed to integrate with popular model providers:
- OpenAI
- Gemini
- LiteLLM
- Ollama (local model support)

## Deployment Options

Recommended stack for this project:
- Rust core library for transformation and provider adaptation logic
- Python FastAPI service as REST/OpenAPI interface
- Rust-to-Python bridge for performance-critical middleware behavior

Deployment options:
- standalone microservice
- middleware service in an API gateway pipeline
- serverless function for event-driven workloads

## Current Implementation Scope

The current implementation targets chat-style transformation only. The Rust core accepts a canonical chat envelope, applies availability-first provider routing, and produces provider-ready payloads for:
- OpenAI
- Gemini
- Ollama

The architecture is capability-oriented so embeddings, image, and audio can be added later without breaking the chat contract.

## Host Integration Contract

`omni_api` is a transformation agent, not a transport client.

- Input: canonical request + provider preference/availability context
- Output: selected provider, provider-ready payload, fallback metadata, warnings, and diagnostics
- Outbound provider HTTP execution: **owned by the host service**

This boundary keeps transport/auth/retry policy in host systems while Rust core stays focused on deterministic payload alignment and routing decisions.
