import { Center, Box, Spacer, Text, Stack, Flex, Button, Input, InputGroup, InputLeftElement } from '@chakra-ui/react'
import { IoSearch as SearchIcon } from 'react-icons/io5'
import Card from '@/components/Card'
import { IoCloudDownloadOutline as DownloadIcon } from 'react-icons/io5'
import { RiEarthFill as PublicIcon } from 'react-icons/ri'

      //name: 'Service',
     // id: 'service',
      //owner: 'patchup',
      //updated: 'a year ago',
      //downloads: '10K+',
      //visibility: 'Public',

const Repository = ({ repository }) => {
  return (
    <Card p={6}>
      <Flex grow={1}>
        <Box>
          
          <Stack direction='column'>
            <Text fontSize='md' fontWeight='bold'>{repository.name}</Text>
            <Stack direction='row'>
              <Text>{repository.owner} /</Text>
              <Text fontWeight='bold'>{repository.id}</Text>
            </Stack>
          </Stack>

        </Box>
        <Spacer />
        <Center>
          <Stack direction='row'>
            <Flex>
              <DownloadIcon />
              <Text>{repository.downloads}</Text>
            </Flex>
            <Flex>
              <PublicIcon />
              <Text>{repository.visibility}</Text>
            </Flex>
          </Stack>
        </Center>
      </Flex>
    </Card>
  )
}

const Repositories = ({ repositories }) => {
  return (
    <Stack direction='column'> 

      <Flex align='baseline' mb={2}>
        <InputGroup>
          <InputLeftElement children={<SearchIcon color='gray' />}/>
          <Input maxWidth='30vw' placeholder='Search by repository name' />
        </InputGroup>
        <Button px={8} colorScheme='blue' size='md'>Add Repository</Button>
      </Flex>

      {repositories.map(repository => (
        <Repository repository={repository} />
      ))}

    </Stack>
  )
}

export default Repositories

