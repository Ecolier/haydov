### June 26, 2025

**What I did**
- Diagram of maps flow (routing & geocoding orchestration)

**What I learned**
- QS can encode complex data as URL params, ensuring readability and caching
- Private services and internet-facing services must live on different networks

**Bugs/Challenges**
- Need to develop principled anti-spoofing techniques to avoid abuse, ensuring legitimacy of discounts while preserving user experience 

**Next up**
- Orchestrate routing & geocoding through a unified maps API
- Use Traefik as an API gateway
- (Look into PlantUML to generate diagrams)

### July 1, 2025

**What I did**
- Added scaffoling for OSM data pulling service
- Added basic Traefik configuration

**What I learned**
- Better to use another microservice to pull new geographical data weekly with hooks to refresh dependent services

**Bugs/Challenges**
- Communication between geographical data pulling service and dependent services

**Next up**
- Pull geographical data from OSM and / or use Osmium
- Integrate it with geocoding and routing services