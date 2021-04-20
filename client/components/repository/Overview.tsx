import { Text, Stack, Grid, GridItem } from '@chakra-ui/react'
import { Table, Thead, Tr, Th, Td } from "@chakra-ui/react"
import Card from '@/components/Card'

const Header = ({ header }) => {
  return (
    <Card>
      <Stack direction='column'>
        
        <Stack direction='row' alignItems='end'>
          <Text fontSize='lg'>{header.org} /</Text>
          <Text fontSize='lg' fontWeight='bold'>{header.project}</Text>
        </Stack>

        <Text>{header.description}</Text>

        <Text>Updated: {header.activity}</Text>

      </Stack>
    </Card>
  )
}

const VersionsTable = ({ versions }) => {
 return ( 
    <Table size='sm' variant='simple'>
      <Thead>
        <Tr>
          <Th>Version</Th>
          <Th>Downloads</Th>
          <Th>Size</Th>
        </Tr>
          {versions.map(version => (
          <Tr key={version.verion}>
            <Td>{version.version}</Td>
            <Td>{version.downloads}</Td>
            <Td>{version.size}</Td>
          </Tr>
          ))}
      </Thead>
    </Table>
  )
}

const Versions = ({ versions }) => {
  return (
    <Card>
      <Stack direction='column'>
      
        <Text fontSize='md' fontWeight='bold' mb={2}>Versions</Text>
        <VersionsTable versions={versions} />

      </Stack>
    </Card>
  )
}

const Builds = ({ builds }) => {
  return (
    <Card>
      <Stack>

        <Text fontSize='md' fontWeight='bold' mb={2}>Builds</Text>

        <Table size='sm' variant='simple'>
          <Thead>
            <Tr>
              <Th>Version</Th>
              <Th>Date</Th>
              <Th>Size</Th>
              <Th>Time</Th>
            </Tr>
              {builds.map(build => (
              <Tr key={build.verion}>
                <Td>{build.version}</Td>
                <Td>{build.date}</Td>
                <Td>{build.size}</Td>
                <Td>{build.time}</Td>
              </Tr>
              ))}
          </Thead>
        </Table>

      </Stack>
    </Card>
  )
}

const Overview = ({ data }) => {
  return (
    <Grid gap={4} templateColumns='repeat(12, 1fr)'>

      <GridItem colSpan={12} rowSpan={1}>
        <Header header={data.header} />
      </GridItem>

      <GridItem colSpan={6} rowSpan={2}>
        <Versions versions={data.versions} />
      </GridItem>

      <GridItem colSpan={6} rowSpan={2}>
        <Builds builds={data.builds} />
      </GridItem>

    </Grid>
  )
}

export default Overview
