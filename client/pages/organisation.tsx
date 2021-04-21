import { Box, HStack, Stack, Text, Center } from '@chakra-ui/react'
import { Tab, Tabs, TabPanel, TabPanels, TabList } from '@chakra-ui/react'
import { Header, Activity, Members, Settings, Repositories } from '@/components/organisation'

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
      owner: 'patchup',
      updated: 'a year ago',
      downloads: '10K+',
      visibility: 'Public',
    }
  ],
  activity: [
  ],
}

const Organisation = () => {
  return (
    <Center p={4}>
      <Stack direction='column'>
        <Header organisation={organisation} />

        <Tabs width='95vw' maxWidth='800'>
          <TabList>
            <Tab>Repositories</Tab>
            <Tab>Members</Tab>
            <Tab>Activity</Tab>
            <Tab>Settings</Tab>
          </TabList>

          <TabPanels>
            <TabPanel>
              <Repositories repositories={organisation.repositories}/>
            </TabPanel>
            <TabPanel>
              <Members members={organisation.members}/>
            </TabPanel>
            <TabPanel>
              <Activity />
            </TabPanel>
            <TabPanel>
              <Settings />
            </TabPanel>
          </TabPanels>
        </Tabs>
      </Stack>

    </Center>
  )
}

export default Organisation
