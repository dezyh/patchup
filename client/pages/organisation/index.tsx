import { Stack, Text, Center } from '@chakra-ui/react'
import { Tab, Tabs, TabPanel, TabPanels, TabList } from '@chakra-ui/react'

const organisation = {
  name: 'Patchup',
  id: 'patchup',
  joined: 'April 20, 2021',
  members: [
    {
      name: 'Ben Mitchell',
      id: 'dezyh',
      permission: 'owner',
    }
  ],
  repositories: [
    {
      name: 'Service',
      id: 'service',
    }
  ],
  activity: [
  ],
}

const Header = ({ organisation }) => {
  return (
    <Stack justifyContent='center' justifyItems='center' minHeight={150} minWidth={1200} direction='column'>
      <Text fontSize='lg' fontWeight='bold'>{organisation.name}</Text>
      <Stack direction='row'>
        <Text>{organisation.id}</Text>
        <Text>{organisation.joined}</Text>
      </Stack>
    </Stack>
  )
}

const Organisation = () => {
  return (
    <Center p={4}>
      <Stack direction='column'>
        <Header organisation={organisation} />

        <Tabs minWidth='70vw' maxWidth='1200'>
          <TabList>
            <Tab>Repositories</Tab>
            <Tab>Members</Tab>
            <Tab>Activity</Tab>
            <Tab>Settings</Tab>
          </TabList>

          <TabPanels>
            <TabPanel>
              <p>Todo</p>
            </TabPanel>
            <TabPanel>
              <p>Todo</p>
            </TabPanel>
            <TabPanel>
              <p>Todo</p>
            </TabPanel>
            <TabPanel>
              <p>Todo</p>
            </TabPanel>
          </TabPanels>
        </Tabs>
      </Stack>

    </Center>
  )
}

export default Organisation
