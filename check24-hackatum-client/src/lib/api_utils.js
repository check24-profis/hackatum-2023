async function api_fetch(domain, url_appendix) {
  let port = 8080;
  let url = `${domain}:${port}/${url_appendix}`;
  let response = await fetch(url);
  if (response.ok) {
    return response.json();
  } else {
    console.error(`Error while fetching ${url_appendix} ->`, response);
    return {};
  }
}

export async function fetch_craftsmen(domain, plz, page, load_state_update) {
  load_state_update(true);
  let response_json = await api_fetch(
    domain,
    `craftsmen?postalcode=${plz}&page=${page}`,
    load_state_update
  );
  load_state_update(false);
  return response_json["data"] ?? [];
}
