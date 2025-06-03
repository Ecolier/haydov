import MapView, { Polyline } from "react-native-maps";
import { decode } from "@/utils/polyline";
import { useEffect, useState } from "react";

const body = {
  locations: [
    {
      lat: 47.365109,
      lon: 8.546824,
      type: "break",
      city: "Zürich",
      state: "Altstadt",
    },
    {
      lat: 46.9480,
      lon: 7.4474,
      type: "break",
      city: "6037 Root",
      state: "Untere Waldstrasse",
    },
  ],
  costing: "auto",
  directions_options: { units: "miles" },
};

export default function TabTwoScreen() {
  const [data, setData] = useState<number[][]>([]);

  useEffect(() => {
    const fetchTrip = async () => {
      try {
        const res = await fetch("http://localhost:8002/route", {
          method: "POST",
          body: JSON.stringify(body),
        });
        if (!res.ok) throw new Error('Network response was not ok');
        const result = await res.json();
        setData(decode(result.trip.legs[0].shape, 6));
      } catch {}
    };
    fetchTrip();
  }, []);

  return (
  <MapView style={{ flex: 1 }}>
    <Polyline coordinates={data.map(coords => ({latitude: coords[0], longitude: coords[1]}))} strokeWidth={5} strokeColor="#005EFF"></Polyline>
  </MapView>
  );
}
