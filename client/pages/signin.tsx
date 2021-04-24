import { useState } from 'react'
import { useDispatch } from 'react-redux'
import { SlideFade, Flex, Heading, Input, Button, useColorModeValue } from '@chakra-ui/react'
import { signInRequest } from '../store/user/actions'

const SignIn = () => {
  const formBackground = useColorModeValue('gray.200', 'gray.700')
  const dispatch = useDispatch()
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const submit = (e) => {
    e.preventDefault()
    dispatch(signInRequest(username, password))
  }
  return (
    <SlideFade in offsetY='20px'>
      <Flex height='100vh' alignItems='center' justifyContent='center'>
        <Flex direction='column' background={formBackground} p={12} rounded={6}>
          <Heading mb={6}>Sign In</Heading>
          <Input onChange={e => setUsername(e.target.value)} placeholder='username' variant='filled' mb={3} type='username' />
          <Input onChange={e => setPassword(e.target.value)} placeholder='password' variant='filled' mb={6} type='password' />
          <Button colorScheme='blue' onClick={(e) => submit(e)}>Sign In</Button>
        </Flex>
      </Flex>
    </SlideFade>
  )
}

export default SignIn
