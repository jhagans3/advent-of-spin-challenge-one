# hurl --error-format long --variable serviceUrl="https://advent-of-spin-challenge-one-eucjhhsi.fermyon.app" submit.hurl
POST https://advent-of-spin2023.fermyon.app/1
{
  "serviceUrl": "{{serviceUrl}}"
}

HTTP 200
