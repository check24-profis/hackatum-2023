async function api_fetch(domain, url_appendix, method="GET", body={}) {
    let port = 8080;
    let url = `${domain}:${port}/${url_appendix}`;
    let params = {
        headers: {},
        method: method
    };
    if(Object.keys(body).length !== 0) {
        params['body'] = JSON.stringify(body);
    }

    let response = await fetch(url, params=params);
    if(response.ok) {
        return response.json();
    } else {
        console.error(`Error while fetching ${url_appendix} ->`, response);
        return {};
    }
}

export async function fetch_craftsmen(domain, plz, page, sort_by, load_state_update) {
    load_state_update(true);
    let response_json = await api_fetch(domain, `craftsmen?postalcode=${plz}&page=${page}&sort_by=${sort_by}`);
    load_state_update(false);
    return response_json['data'] ?? [];
}

export async function update_craftsman(domain, id, maxDrivingDistance, profilePictureScore, profileDescriptionScore) {
    await api_fetch(domain, `craftman/${id}`, 'PATCH', {
        'maxDrivingDistance': maxDrivingDistance,
        'profilePictureScore': profilePictureScore,
        'profileDescriptionScore': profileDescriptionScore,
    });
}