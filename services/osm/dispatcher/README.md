# OSM Dispatcher

A service for automating the download and distribution of OpenStreetMap (OSM) data from external providers (e.g., [Geofabrik](https://download.geofabrik.de/)), with seamless integration to S3 and event-driven updates for downstream services.

## Features

- **Automated OSM Data Fetching:** Downloads OSM extracts from a configurable provider on a schedule (default: weekly, configurable via `crontab`).
- **S3 Integration:** Uploads the latest OSM data to a specified S3 bucket.
- **Event Notification:** Triggers an S3 event upon successful upload, enabling other services to stay up-to-date.
- **Configurable:** Easily adjust download frequency, provider URL, and S3 settings.

## Usage

1. **Configure the Service:**
  - Set the OSM provider URL, S3 bucket details, and desired schedule in the configuration file or environment variables.
  - Select the regions suited for your application's needs.

2. **Set Up Scheduling:**
  - By default, the service runs weekly. Adjust the schedule using `crontab` as needed.

3. **S3 Event Handling:**
  - Ensure your S3 bucket is configured to send events (e.g., via SNS, SQS, or Lambda) when new data is uploaded.

## Environment Variables

| Variable            | Description                        |
|---------------------|------------------------------------|
| `OSM_PROVIDER_URL`  | URL to download OSM data           |
| `S3_BUCKET`         | Target S3 bucket name              |
| `AWS_ACCESS_KEY_ID` | AWS access key                     |
| `AWS_SECRET_ACCESS_KEY` | AWS secret key                 |
