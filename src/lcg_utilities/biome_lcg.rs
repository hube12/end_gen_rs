pub fn next(world_seed:i64,salt:i64)->i64{
    return world_seed*( world_seed * 6364136223846793005i64 + 1442695040888963407i64) + salt;
}