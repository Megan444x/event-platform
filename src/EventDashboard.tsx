import React, { useState, useEffect } from 'react';
import { Calendar, momentLocalizer } from 'react-big-calendar';
import moment from 'moment';
import 'react-big-calendar/lib/css/react-big-calendar.css';

const API_BASE_URL = process.env.REACT_APP_API_BASE_URL;

const calendarLocalizer = momentLocalizer(moment);

interface IEvent {
  id: number;
  title: string;
  start: Date;
  end: Date;
  sessions?: ISession[];
}

interface ISession {
  id: number;
  eventId: number;
  speaker: string;
  location: string;
  startTime: Date;
  endTime: Date;
}

interface IAttendee {
  id: number;
  name: string;
  email: string;
  sessionId: number;
}

const EventCalendarDashboard: React.FC = () => {
  const [allEvents, setAllEvents] = useState<IEvent[]>([]);
  const [allSessions, setAllSessions] = useState<ISession[]>([]);
  const [allAttendees, setAllAttendees] = useState<IAttendee[]>([]);

  useEffect(() => {
    fetch(`${API_BASE_URL}/events`)
      .then((res) => res.json())
      .then(setAllEvents);
  }, []);

  useEffect(() => {
    fetch(`${API_BASE_URL}/sessions`)
      .then((res) => res.json())
      .then(setAllSessions);
  }, []);

  useEffect(() => {
    fetch(`${API_BASE_URL}/attendees`)
      .then((res) => res.json())
      .then(setAllAttendees);
  }, []);

  const handleEventSelection = (selectedEvent: IEvent) => {
    console.log(`Selected event: ${selectedEvent.title}`);
  };

  const customizeEventStyle = () => {
    return {
      style: {
        backgroundColor: 'lightblue',
      },
    };
  };

  return (
    <div>
      <Calendar
        localizer={calendarLocalizer}
        events={allEvents.map(event => ({...event, start: new Date(event.start), end: new Date(event.end)}))}
        startAccessor="start"
        endAccessor="end"
        style={{ height: 500 }}
        onSelectEvent={handleEventSelection}
        eventPropGetter={customizeEventStyle}
      />
    </div>
  );
};

export default EventCalendarDashboard;