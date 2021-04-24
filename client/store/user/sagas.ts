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
    token = yield call(signin, username, password)
    console.log(token)
    yield put(signInSuccess(token)) 

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
    const { username, password } = yield take(ActionType.SIGN_IN_REQUEST)
    const task = yield fork(signinFlow, username, password)
    const action = yield take([ActionType.SIGN_IN_SUCCESS, ActionType.SIGN_IN_FAILURE])
    if (action.type === ActionType.SIGN_OUT_REQUEST) yield cancel(task)
    yield call(signout)
  }
}

export default signinWatcher
