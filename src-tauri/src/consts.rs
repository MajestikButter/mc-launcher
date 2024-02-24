pub const RELEASE_SECURITY_ID: &str =
  "S-1-15-2-1958404141-86561845-1752920682-3514627264-368642714-62675701-733520436";
pub const PREVIEW_SECURITY_ID: &str =
  "S-1-15-3-424268864-5579737-879501358-346833251-474568803-887069379-4040235476";

pub const RELEASE_DESTINATION: &str =
  "%localappdata%/Packages/Microsoft.MinecraftUWP_8wekyb3d8bbwe/LocalState/games/com.mojang";
pub const PREVIEW_DESTINATION: &str = "%localappdata%/Packages/Microsoft.MinecraftWindowsBeta_8wekyb3d8bbwe/LocalState/games/com.mojang";

pub const MINECRAFT: &str = "Microsoft.MinecraftUWP_8wekyb3d8bbwe";
pub const MINECRAFT_PREVIEW: &str = "Microsoft.MinecraftWindowsBeta_8wekyb3d8bbwe";

pub const EXCLUDE_IN_PACKAGE: &[&str] = &[
  "AppxBlockMap.xml",
  "AppxMetadata/CodeIntegrity.cat",
  "AppxMetadata",
  "AppxSignature.p7x",
  "[Content_Types].xml",
];
