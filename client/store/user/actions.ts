import { ActionType } from './types'

const signInRequest = (username, password) => {
  return {
    type: ActionType.SIGN_IN_REQUEST,
    username,
    password,
  }
}

const signInSuccess = (token) => {
  return {
    type: ActionType.SIGN_IN_SUCCESS,
    token,
  }
}

const signUpRequest = (username, password, email, firstname, lastname) => {
  return {
    type: ActionType.SIGN_UP_REQUEST,
    username,
    password,
    email,
    firstname,
    lastname,
  }
}

const signUpSuccess = (token) => {
  return {
    type: ActionType.SIGN_UP_SUCCESS,
    token,
  }
}

const signOutRequest = () => {
  return {
    type: ActionType.SIGN_OUT_REQUEST,
  }
}

export { signOutRequest, signInRequest, signUpRequest, signUpSuccess, signInSuccess }
