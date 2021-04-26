import { Text, Input, Stack, Container, } from '@chakra-ui/react'
import Card from '@/components/Card'

const Patch = ({ patch }) => {
  return (
    <Card p={4} >
      <Stack direction='column'>
        <Text>Version</Text>
        <Text>{patch.version}</Text>
        <Text>Created: {patch.date}</Text>
      </Stack>
    </Card>
  )
}


const Patches = ({ patches }) => {
  return (

    <Stack spacing={4} direction='column'>

      <Input placeholder='filter'/>

      {patches.map(patch => (
        <Patch patch={patch} />
      ))}

    </Stack>
  )
}

export default Patches
