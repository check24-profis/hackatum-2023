const getMappedCraftsman = (craftsman) => ({
  id: craftsman.id,
  name: `${craftsman.first_name} ${craftsman.last_name}`,
  postalCode: craftsman.postalcode,
  rankingScore: craftsman.ranking_score
});

module.exports = {
  getMappedCraftsman,
}