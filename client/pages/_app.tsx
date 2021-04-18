import { Provider } from 'react-redux'
import { useStore } from '../store'
import { ChakraProvider } from '@chakra-ui/react'
import theme from '../styles/theme'

export default function App({ Component, pageProps }) {
  const store = useStore(pageProps.initialReduxState)

  return (
    <Provider store={store}>
      <ChakraProvider theme={theme}>
        <Component {...pageProps} />
      </ChakraProvider>
    </Provider>
  )
}
