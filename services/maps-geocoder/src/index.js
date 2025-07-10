var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
import fastify from "fastify";
const app = fastify();
app.post("/update-osm", (request, reply) => __awaiter(void 0, void 0, void 0, function* () {
    try {
        // Simulate the OSM update process
        console.log("Starting OSM update process...");
        // Here you would typically call your Rust service or any other logic
        // For now, we just simulate a delay
        yield new Promise(resolve => setTimeout(resolve, 2000));
        console.log("OSM update completed successfully.");
        return reply.status(200).send({ message: "OSM update completed successfully." });
    }
    catch (error) {
        console.error("Error during OSM update:", error);
        return reply.status(500).send({ error: "Failed to update OSM data." });
    }
}));
app.listen({ port: 3000 }, (err, address) => {
    if (err) {
        console.error(err);
        process.exit(1);
    }
    console.log(`Server listening at ${address}`);
});
