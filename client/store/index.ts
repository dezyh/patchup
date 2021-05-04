import { createStore, applyMiddleware } from 'redux'
import { useMemo } from 'react'
import { composeWithDevTools } from 'redux-devtools-extension'
import createSagaMiddleware from 'redux-saga'
import rootSaga from './saga'
import { initialState as userState } from './user/types'
import { combineReducers } from 'redux'
import user from './user/reducer' 

let store

const initialState = {
  user: userState,
}

const rootReducer = combineReducers({
  user,
})

const sagaMiddleware = createSagaMiddleware()

function initStore(preloadedState = initialState) {
  const store = createStore(
    rootReducer,
    preloadedState,
    composeWithDevTools(applyMiddleware(sagaMiddleware))
  )
  sagaMiddleware.run(rootSaga)
  return store
}

export const initializeStore = (preloadedState) => {
  let _store = store ?? initStore(preloadedState)

  if (preloadedState && store) {
    _store = initStore({
      ...store.getState(),
      ...preloadedState,
    })
    store = undefined
  }

  if (typeof window === 'undefined') return _store

  if (!store) store = _store

  return _store
}

export function useStore(initialState) {
  const store = useMemo(() => initializeStore(initialState), [initialState])
  return store
}
//export const useStore = () => {
//  const initialState = {
//    'user': userState,
//  }
//  
//  const store = createStore(
//    //persistReducer(persistConfig, rootReducer),
//    rootReducer,
//    initialState,
//    composeWithDevTools(applyMiddleware(sagaMiddleware))
//  )
//
//  // const persistor = persistStore(store)
//
//  sagaMiddleware.run(rootSaga)
//  return store
//}

export type RootState = ReturnType<typeof rootReducer>
