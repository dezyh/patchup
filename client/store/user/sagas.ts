import { take, fork, cancel, call, put, cancelled } from 'redux-saga/effects'
import { signin, signup } from './api'
import { ActionType } from './types'
import { signInSuccess } from './actions'

function* signout() {
  yield put({ type: ActionType.SIGN_OUT_REQUEST })
}

function* signinFlow(username: string, password: string) {
  let token
  try {
    const response = yield call(signin, username, password)
    yield put(signInSuccess(response.data.token)) 

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
    if (action.type === ActionType.SIGN_OUT_REQUEST) {
      yield cancel(task)
    } else {
      yield call(signout)
    }
  }
}

export default signinWatcher
