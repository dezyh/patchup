const url = '/api/user'

const signin = async (username: string, password: string) => {
  return fetch(`${url}/signin`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      username,
      password,
    })
  })
  .then(response => response.json())
  .then(json => json)
  .catch(error => { throw error })
}

const signup = async (username: string, password: string, email: string, firstname: string, lastname: string) => {
  return fetch(`${url}/signup`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      username,
      password,
      email,
      firstname,
      lastname,
    })
  })
  .then(response => response.json())
  .then(json => json)
  .catch(error => { throw error })
}

export { signin, signup }
