const { HttpClient } = require("longport");

async function main() {
  let cli = HttpClient.fromEnv();
  let resp = await cli.request("get", "/v1/trade/execution/today");
  console.log(resp);
}

Promise.all([main()]).catch((err) => console.error(err));
