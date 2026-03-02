const { Config, OAuth } = require("longport");

async function main() {
  const oauth = new OAuth("your-client-id");
  const token = await oauth.authorize((url) => {
    console.log("Open this URL to authorize:", url);
  });
  console.log("Access token:", token.accessToken);
  console.log("Expires at:", token.expiresAt);

  const config = Config.fromOauth("your-client-id", token.accessToken);
  console.log("Config created:", config);
}

Promise.all([main()]).catch((err) => console.error(err));
