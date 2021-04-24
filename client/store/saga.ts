import { all } from 'redux-saga/effects'
import UserSaga from './user/sagas'

export default function* saga() {
  yield all([
    UserSaga(),
  ])
}
