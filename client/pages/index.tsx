import { Center, Stack, Text, Heading } from '@chakra-ui/react'

const Main = () => {
  return (
    <Center>
      <Stack direction='column'>
        <Center>
          <Heading>Patchup</Heading>
        </Center>
        <Text>A cloud service to create and distribute binary patches.</Text>
      </Stack>
    </Center>
  )
}

export default Main
