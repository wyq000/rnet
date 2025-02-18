import pytest
import rnet
from rnet import Version


@pytest.mark.asyncio
async def test_base_url():
    base_url = "https://httpbin.org"
    client = rnet.Client(tls_info=True, base_url=base_url)
    url = "/anything"
    response = await client.get(url)
    assert response.url == base_url + url
