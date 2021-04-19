import { SlideFade, Flex, Heading, Input, Button, useColorModeValue } from '@chakra-ui/react'

const SignIn = () => {
  const formBackground = useColorModeValue('gray.200', 'gray.700')
  return (
    <SlideFade in offsetY='20px'>
      <Flex height='100vh' alignItems='center' justifyContent='center'>
        <Flex direction='column' background={formBackground} p={12} rounded={6}>
          <Heading mb={6}>Sign In</Heading>
          <Input placeholder='email' variant='filled' mb={3} type='email' />
          <Input placeholder='password' variant='filled' mb={6} type='password' />
          <Button colorScheme='blue'>Sign In</Button>
        </Flex>
      </Flex>
    </SlideFade>
  )
}

export default SignIn
