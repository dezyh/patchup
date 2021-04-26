import { Box, HStack, Stack, Text, Center } from '@chakra-ui/react'
import { IoEarth as WorldIcon } from 'react-icons/io5'

const Header = ({ organisation }) => {
  return (
    <HStack>
      <Box p={4}>
        <WorldIcon size='75' />
      </Box>
      <Stack justifyContent='center' justifyItems='center' minHeight={125} direction='column'>
        <Text fontSize='lg' fontWeight='bold'>{organisation.name}</Text>
        <Stack direction='row'>
          <Text>{organisation.id}</Text>
          <Text>{organisation.joined}</Text>
        </Stack>
      </Stack>
    </HStack>
  )
}

export default Header
