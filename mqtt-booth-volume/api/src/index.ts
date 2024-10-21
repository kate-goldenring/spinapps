import { ResponseBuilder , Sqlite, Router} from "@fermyon/spin-sdk";

export async function handler(req: Request, res: ResponseBuilder) {
  let conn = Sqlite.openDefault();
  let result = conn.execute("SELECT * FROM noise_log", []);
  let items = result.rows.map(row => {
    return {
        source: row["source"],
        volume: Number(row["volume"]),
        timestamp: row["timestamp"],
    }
});
  res.set({ "content-type": "application/json" });
  res.send(JSON.stringify(items));
}