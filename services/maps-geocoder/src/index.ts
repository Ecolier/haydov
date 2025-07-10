import fastify from "fastify";

const app = fastify();
app.post("/update-osm", async (request, reply) => {
  try {
    // Simulate the OSM update process
    console.log("Starting OSM update process...");

    // Here you would typically call your Rust service or any other logic
    // For now, we just simulate a delay
    await new Promise(resolve => setTimeout(resolve, 2000));

    console.log("OSM update completed successfully.");
    return reply.status(200).send({ message: "OSM update completed successfully." });
  } catch (error) {
    console.error("Error during OSM update:", error);
    return reply.status(500).send({ error: "Failed to update OSM data." });
  }
});

app.listen({ port: 3000 }, (err, address) => {
  if (err) {
    console.error(err);
    process.exit(1);
  }
  console.log(`Server listening at ${address}`);
});
