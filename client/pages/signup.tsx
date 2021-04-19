import { Stack, SlideFade, Flex, Heading, Input, Button, useColorModeValue } from '@chakra-ui/react'

const SignUp = () => {
  const formBackground = useColorModeValue('gray.200', 'gray.700')
  return (
    <SlideFade in offsetY='20px'>
      <Flex height='100vh' alignItems='center' justifyContent='center'>
        <Flex direction='column' background={formBackground} p={12} rounded={6} maxWidth='sm'>
          <Heading mb={6}>Sign Up</Heading>
          <Input placeholder='username' variant='filled' mb={3} type='username' />
          <Input placeholder='email' variant='filled' mb={3} type='email' />
          <Input placeholder='password' variant='filled' mb={3} type='password' />
          <Stack direction='row' mb={3}>
            <Input placeholder='firstname' variant='filled' mb={3} type='firstname' />
            <Input placeholder='lastname' variant='filled' mb={3} type='lastname' />
          </Stack>
          <Button colorScheme='blue'>Sign Up</Button>
        </Flex>
      </Flex>
    </SlideFade>
  )
}

export default SignUp
