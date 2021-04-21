import Link from 'next/link'
import { Center, Stack, Text, Heading, Button } from '@chakra-ui/react'
import OctopusLogo from '../components/OctopusLogo'

const Main = () => {
  return (
    <Center p={12}>
      <Stack direction='column'>

        <Center p={3}>
          <OctopusLogo width='20vw'/>
        </Center>
        
        <Center>
          <Heading>Patchup</Heading>
        </Center>
        <Text>A cloud service to create and distribute binary patches.</Text>

        <Stack pt={3} direction='row'>
          <Link href='/signup'>
            <Button colorScheme='blue'>Get Started</Button>
          </Link>
          <Button>GitHub</Button>
        </Stack>

      </Stack>
    </Center>
  )
}

export default Main
