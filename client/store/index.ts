import { createStore, applyMiddleware } from 'redux'
import { composeWithDevTools } from 'redux-devtools-extension'
import createSagaMiddleware from 'redux-saga'
import { rootReducer, RootState } from './reducer'
import { rootSaga } from './saga'

const sagaMiddleware = createSagaMiddleware()

export const useStore = (initialState?: RootState) => {
  const store = createStore(
    rootReducer,
    initialState,
    composeWithDevTools(applyMiddleware(sagaMiddleware))
  )
  sagaMiddleware.run(rootSaga)
  return store
}
