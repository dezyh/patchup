import { Center, Text, Stack, Flex, useColorModeValue, Grid, GridItem } from '@chakra-ui/react'
import { Tab, Tabs, TabPanel, TabPanels, TabList } from '@chakra-ui/react'
import Overview from '@/components/repository/Overview'
import Patches from '@/components/repository/Patches'

const data = {
  header: {
    org: 'dezyh',
    project: 'jungle-book',
    description: 'A good description about the project',
    activity: 'a year ago',
  },
  versions: [
    {
      version: '1.2.1',
      downloads: 412,
      size: '14kb',
      date: '2 days ago'
    },
    {
      version: '1.2.0',
      downloads: 873,
      size: '18kb',
      date: '20/4/21',
    },
  ],
  builds: [
    {
      version: '1.2.1',
      date: '20/04/2021',
      time: '121 sec',
      size: '14kb',
    },
    {
      version: '1.2.1',
      date: '20/04/2021',
      time: '121 sec',
      size: '14kb',
    },
    {
      version: '1.2.1',
      date: '20/04/2021',
      time: '121 sec',
      size: '14kb',
    },
    {
      version: '1.2.1',
      date: '20/04/2021',
      time: '121 sec',
      size: '14kb',
    },
  ]
}




const Repository = () => {

  return (
    <Center mt={4}>

  <Tabs minWidth='70vw' maxWidth='1200'>
        <TabList>
          <Tab>Overview</Tab>
          <Tab>Patches</Tab>
          <Tab>Builds</Tab>
          <Tab>Settings</Tab>
        </TabList>

        <TabPanels>
          <TabPanel>
            <Overview data={data} />
          </TabPanel>
          <TabPanel>
            <Patches patches={data.versions} />
          </TabPanel>
          <TabPanel>
            <p>todo</p>
          </TabPanel>
          <TabPanel>
            <p>todo</p>
          </TabPanel>
        </TabPanels>
      </Tabs>

    </Center>
  )
}

export default Repository
