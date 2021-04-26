import { take, fork, cancel, call, put, cancelled } from 'redux-saga/effects'
import { signin, signup } from './api'
import { ActionType } from './types'
import { signInSuccess, signUpSuccess } from './actions'
import Router from 'next/router'

function* signout() {
  yield put({ type: ActionType.SIGN_OUT_REQUEST })
}

function* signupFlow(username: string, password: string, email: string, firstname: string, lastname: string) {
  let token = ''
  try {
    // Make the signup request to the server and store the response
    const response = yield call(signup, username, password, email, firstname, lastname)
    // Handle successful and unsuccessful responses
    if (response.data) {
      yield put(signUpSuccess(response.data.token))
      yield Router.push('/')
    } else {
      yield put({ type: ActionType.SIGN_UP_FAILURE })
    }
  } catch (error) {
    // Handle any other errors
    yield put({ type: ActionType.SIGN_UP_FAILURE })
  } finally {
    // Handle cancelled tasks
    if (yield cancelled()) {
      console.log('Cancelled sign up flow')
    }
  }

  return token
}

function* signinFlow(username: string, password: string) {
  let token = ''
  try {
    const response = yield call(signin, username, password)

    if (response.data) {
      yield put(signInSuccess(response.data.token))
      yield Router.push('/')
    } else {
      yield put({ type: ActionType.SIGN_IN_FAILURE })
    }

  } catch (error) {
    yield put({ type: ActionType.SIGN_IN_FAILURE })

  } finally {
    if (yield cancelled()) {
      console.log('Cancelled sign in flow')
    }
  }

  return token
}

function* signinWatcher() {
  while (true) {
    // Sign the user in when they make the request
    const { username, password } = yield take(ActionType.SIGN_IN_REQUEST)
    const task = yield fork(signinFlow, username, password)

    // Look for any signout actions
    const action = yield take([ActionType.SIGN_OUT_REQUEST, ActionType.SIGN_IN_FAILURE])
    if (action.type === ActionType.SIGN_OUT_REQUEST) yield cancel(task)
  }
}

function* signupWatcher() {
  while (true) {
    // Take any sign up requests and fork a new sign up action
    const { username, password, email, firstname, lastname } = yield take(ActionType.SIGN_UP_REQUEST)
    const task = yield fork(signupFlow, username, password, email, firstname, lastname)

    // Look for any resulting actions and handle them
    const action = yield take([ActionType.SIGN_UP_FAILURE, ActionType.SIGN_OUT_REQUEST])
    if (action.type === ActionType.SIGN_OUT_REQUEST) cancel(task)
  }
}

function* watchers() {
  yield fork(signupWatcher)
  yield fork(signinWatcher)
}


export default watchers
