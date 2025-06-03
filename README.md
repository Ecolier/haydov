# 🌍 Haydov

**Plan. Move. Connect.**  
*The travel and hospitality app built for the modern nomad.*

---

## 🚀 What is Haydov?

**Haydov** is a map-based travel platform designed for digital nomads and long-term travelers. It helps users plan their journey across regions or continents, automatically identifying:

- 🛏️ Hosts and homestays along their path  
- 🧭 Walking, cycling, or driving routes using self-hosted Valhalla  
- 🎉 Local events happening during predicted stopover dates  
- 🤝 Community meetups and shared legs of the journey

The name **“Haydov”** comes from the Uzbek word **“haydovchi”**, meaning “driver” or “one who guides.” It reflects our mission to guide independent travelers through meaningful and spontaneous routes.

---

## ✨ Features

- 🗺️ **Interactive Map with GPX Import**: Upload your journey, Haydov finds stops and events on your route.
- 🧠 **AI-Powered Pace Prediction**: Adjusts your estimated timing based on your travel habits.
- 🔍 **Smart Search with Geocoding**: Find cities, landmarks, and hosts via OpenCage or LocationIQ.
- 📦 **Offline-Ready Routing**: Valhalla-powered navigation optimized for foot, cycle, and car modes.
- 🏕️ **Community-Driven Hosting**: Hosts can offer shelter, food, or local knowledge.
- 📆 **Event Discovery by Date and Location**: Know what's happening during your stay, automatically.

---

## 🛠️ Tech Stack

| Component          | Tool / Service             |
|--------------------|----------------------------|
| **Frontend**       | React Native (Expo)        |
| **Backend API**    | Node.js / Express          |
| **Routing Engine** | [Valhalla](https://github.com/valhalla/valhalla) (self-hosted) |
| **Geocoding**      | OpenCage or LocationIQ API |
| **Map Data**       | OpenStreetMap              |
| **Database**       | PostgreSQL / PostGIS       |
| **Caching**        | Redis                      |
| **Hosting**        | Railway / Fly.io / VPS     |

---

## 🧪 MVP Scope

- Upload and parse GPX files  
- Generate routes and time estimates using Valhalla  
- Display hosts and events on a Leaflet or MapLibre map  
- Geocode locations with cache fallback  
- Simple mobile-friendly UI (Expo)

---

## 🔧 Setup

1. **Clone this repo**
   ```bash
   git clone https://github.com/ecolier/haydov.git
   cd haydov
